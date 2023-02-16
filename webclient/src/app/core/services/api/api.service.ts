import {Injectable} from '@angular/core';
import {HttpClient, HttpErrorResponse, HttpEventType} from "@angular/common/http";
import {
  audit,
  catchError,
  first,
  interval,
  map,
  Observable,
  of,
  Subject,
  Subscription,
  switchMap,
  takeUntil
} from "rxjs";
import {
  ApiFailure,
  Auth,
  Bucket,
  CreatePostData,
  Dimensions,
  DocumentData,
  Media,
  MediaType,
  Page,
  PageParams,
  Post,
  PostDetail,
  PostItemDetail, PostSearchQuery,
  SearchPost, SearchPostItem,
  Tag,
  Upload
} from "@core/models";
import {environment} from "@src/environments/environment";


export interface UploadProgress {
  type: 'progress' | 'complete'
  body: any | undefined,
  bytesPerSec: number | undefined,
  progress: number | undefined,
  uploadedBytes: number | undefined,
  content: Media | undefined,
  thumbnail: Media | undefined,
}

interface QueuedFile {
  file: File,
  auth: Auth,
  subject: Subject<UploadProgress>,
  cancellationToken: Observable<any>
}

@Injectable({
  providedIn: 'root'
})
export class ApiService {

  constructor(private client: HttpClient) {
  }

  public getAllBuckets(): Observable<Bucket[]> {
    return this.get('/buckets').pipe(
      map((json) => {
        return json.map((x: any) => this.mapBucket(x));
      })
    )
  }

  public getBucketById(id: number): Observable<Bucket> {
    return this.get(`/buckets/${id}`).pipe(
      map((json) => {
        return this.mapBucket(json);
      })
    )
  }

  public getPostById(auth: Auth, id: number): Observable<PostDetail> {
    return this.authenticatedGet(auth, `/posts/${id}`).pipe(
      map((json) => {
        return this.mapPostDetail(json)
      })
    )
  }

  public deletePost(auth: Auth, id: number): Observable<any> {
    return this.authenticatedDelete(auth, `/posts/${id}`)
  }

  public searchTags(auth: Auth, pageParams: PageParams, query: string): Observable<{ tags: Tag[], page: Page }> {
    return this.authenticatedGet(auth, `/tags?offset=${encodeURIComponent(pageParams.offset)}&size=${encodeURIComponent(pageParams.pageSize)}&query=${encodeURIComponent(query)}`).pipe(
      map((json) => {
        return {
          tags: json.data.map((t: any) => this.mapTag(t)),
          page: this.mapPage(json)
        }
      })
    )
  }

  public searchPostItems(auth: Auth, postId: number, pageParams: PageParams): Observable<{ items: SearchPostItem[], page: Page }> {
    return this.authenticatedGet(auth, `/posts/${encodeURIComponent(postId)}/items?offset=${encodeURIComponent(pageParams.offset)}&size=${encodeURIComponent(pageParams.pageSize)}`).pipe(
      map((json) => {
        return {
          items: json.data.map((t: any) => this.mapSearchPostItem(auth, t)),
          page: this.mapPage(json)
        }
      })
    )
  }

  public createTag(auth: Auth, name: string, groupId: number | null): Observable<Tag> {
    return this.authenticatedPost(auth, '/tags', {
      name,
      group: groupId
    }).pipe(
      map(json => this.mapTag(json))
    );
  }


  public removeTag(auth: Auth, id: number): Observable<any> {
    return this.authenticatedDelete(auth, `/tags/${id}`);
  }

  public getPostItemById(auth: Auth, postId: number, position: number): Observable<PostItemDetail> {
    return this.authenticatedGet(auth, `/posts/${postId}/items/${position}`).pipe(
      map((json) => this.mapPostItem(auth, json))
    )
  }

  public getMediaById(auth: Auth, id: number): Observable<Media> {
    return this.authenticatedGet(auth, `/media/${id}`).pipe(
      map((json) => {
        return this.mapMedia(auth, json)
      })
    )
  }

  public searchPosts(auth: Auth, query: PostSearchQuery, pageParams: PageParams): Observable<{ posts: SearchPost[], page: Page }> {
    let tagItems = query.items.filter(x => x.type == 'tag');
    let tagIds = '';

    if (tagItems.length > 0) {
      tagIds = '&tags=' + tagItems.map(x => x.tag.id).join(',');
    }

    let queryStr = `${tagIds}`;


    return this.authenticatedGet(auth, `/posts?offset=${encodeURIComponent(pageParams.offset)}&size=${encodeURIComponent(pageParams.pageSize)}${queryStr}`).pipe(
      map((json) => {
        return {
          posts: json.data.map((p: any) => this.mapSearchPost(auth, p)),
          page: this.mapPage(json)
        }
      })
    )
  }

  public createPost(auth: Auth, postInfo: CreatePostData, files: Upload[]): Observable<{ batchId: number, posts: Post[] }> {
    return this.authenticatedPost(auth, '/posts', {
      title: postInfo.title,
      description: postInfo.description,
      source: postInfo.source,
      flatten: postInfo.flatten,
      tag_ids: postInfo.tags.map(x => x.id),
      items: files.filter(x => x.content !== null).map(upload => ({
        content_id: upload.content?.id,
        metadata: {
          original_filename: upload.file.name,
          original_directory: null,
          original_modified_at: new Date(upload.file.lastModified).toISOString(),
          original_accessed_at: null,
        }
      }))
    }).pipe(
      map(json => {
        return {
          batchId: json.batch.id,
          posts: json.posts.map((x: any) => this.mapPost(x))
        }
      })
    );
  }

  public login(bucketId: number, password: string | null, privateSession: boolean): Observable<Auth> {
    return this.post(`/buckets/${bucketId}/auth`, {password}).pipe(
      map((json) => {
        return this.mapAuth(bucketId, privateSession, json);
      })
    )
  }

  public logout(auth: Auth): Observable<any> {
    return this.authenticatedPost(auth, '/buckets/logout', {});
  }

  // downloads

  private queue: QueuedFile[] = [];
  private isRunning = false;
  private workerPromise: null | Promise<void> = null;

  public uploadFile(auth: Auth, file: File, cancellationToken: Observable<any> | null): Observable<UploadProgress> {
    let subject = new Subject<UploadProgress>();

    this.queue.push({
      file,
      subject,
      auth,
      cancellationToken: cancellationToken || new Subject()
    });

    this.runQueue();

    return subject.asObservable();
  }

  private runQueue() {
    if (this.workerPromise) {
      return;
    }

    this.workerPromise = this.runQueueLoop();
  }

  private async runQueueLoop() {
    let next: QueuedFile | undefined = undefined;

    while (next = this.queue.shift()) {
      await this.uploadQueuedFile(next);
    }

    this.workerPromise = null;
  }

  private uploadQueuedFile(file: QueuedFile): Promise<void> {
    return new Promise((resolve) => {
      let prevTime = new Date();
      let prevLoaded = 0;
      let cancelSubscription: Subscription | null = null;

      let resolved = false;

      let subscription = this.authenticatedPost(file.auth, '/content', file.file, {
        reportProgress: true,
        observe: 'events',
        responseType: 'json'
      }).pipe(
        takeUntil(file.cancellationToken),
        catchError(err => {
          file.subject.error(err);
          resolve();
          cancelSubscription?.unsubscribe();
          throw err;
        }),
        audit(() => interval(500))
      )
        .subscribe(event => {
          if (event.type == HttpEventType.UploadProgress) {
            let now = new Date();
            let secDiff = (now.getTime() - prevTime.getTime()) / 1000;

            file.subject.next({
              type: 'progress',
              body: undefined,
              bytesPerSec: (event.loaded - prevLoaded) / secDiff,
              progress: event.loaded / event.total! * 100.0,
              uploadedBytes: event.loaded!,
              content: undefined,
              thumbnail: undefined
            });

            prevTime = now;
            prevLoaded = event.loaded;

            if (event.loaded == event.total!) {
              resolve();
              resolved = true;
            }

          } else if (event.type == HttpEventType.Response) {
            file.subject.next({
              type: 'complete',
              body: event.body,
              bytesPerSec: undefined,
              progress: undefined,
              uploadedBytes: undefined,
              content: this.mapMedia(file.auth, event.body.content.obj),
              thumbnail: this.mapMedia(file.auth, event.body.thumbnail.obj),
            });

            file.subject.complete();
            cancelSubscription?.unsubscribe();

            if (!resolved) {
              resolve();
            }
          }
        });


      cancelSubscription = file.cancellationToken.pipe(first()).subscribe(() => {
        subscription.unsubscribe();
        file.subject.complete();
        resolve();
      })
    });
  }

  // http basics

  private get(url: string, options?: any): Observable<any> {
    return this.pipeRequest(this.client.get(`${environment.api}${url}`, options));
  }

  private authenticatedGet(auth: Auth, url: string): Observable<any> {
    return this.get(`/buckets/${auth.bucketId}${url}`, this.authRequestOptions(auth));
  }

  private post(url: string, data: any, options?: any): Observable<any> {
    return this.pipeRequest(this.client.post(`${environment.api}${url}`, data, options));
  }

  private authenticatedPost(auth: Auth, url: string, data: any, options?: any): Observable<any> {
    return this.post(`/buckets/${auth.bucketId}${url}`, data, {...this.authRequestOptions(auth), ...options});
  }

  private authenticatedDelete(auth: Auth, url: string, options?: any): Observable<any> {
    return this.delete(`/buckets/${auth.bucketId}${url}`, {...this.authRequestOptions(auth), ...options});
  }

  private put(url: string, data: any): Observable<any> {
    return this.pipeRequest(this.client.put(`${environment.api}${url}`, data));
  }

  private delete(url: string, options?: any): Observable<any> {
    return this.pipeRequest(this.client.delete(`${environment.api}${url}`, options));
  }

  private authRequestOptions(auth: Auth): any {
    return {
      headers: {
        'Authorization': auth.token + '',
      }
    }
  }

  private pipeRequest(req: Observable<any>): Observable<any> {
    return req.pipe(
      catchError(async (err: HttpErrorResponse) => {
        if (typeof err.error?.message == 'string' && typeof err.error?.status == 'number' && typeof err.error?.status_text == 'string') {
          throw new ApiFailure(err.error.message, err.error.inner_error, err.error.status, err.error.status_text);
        }

        throw err;
      })
    );
  }

  private mapBucket(json: any): Bucket {
    return new Bucket(json.id, json.name, json.password_protected, json.encrypted);
  }

  private mapAuth(bucketId: number, privateSession: boolean, json: any): Auth {
    return new Auth(bucketId, json.token, privateSession);
  }

  private mapPage(json: any): Page {
    return new Page(
      new PageParams(json.page_size, json.page_number),
      json.total_row_count
    )
  }

  private mapSearchPost(auth: Auth, json: any): SearchPost {
    return new SearchPost(
      json.post.id,
      json.post.source,
      json.post.title,
      json.post.description,
      new Date(json.post.created_at),
      json.item_count,
      json.contains_image,
      json.contains_video,
      json.contains_moving_image,
      json.duration,
      json.thumbnail == null ? null : this.mapMedia(auth, json.thumbnail),
    )
  }

  private mapMedia(auth: Auth, json: any): Media {
    let dimensions = null;
    let duration = null;
    let documentData = null;
    let videoEncoding = null;
    let mediaType: MediaType = 'unknown';

    if (!!json.metadata.Image) {
      dimensions = new Dimensions(json.metadata.Image.dims.width, json.metadata.Image.dims.height);
      mediaType = 'image';
    } else if (!!json.metadata.Video) {
      dimensions = new Dimensions(json.metadata.Video.dims.width, json.metadata.Video.dims.height);
      duration = json.metadata.Video.duration;
      videoEncoding = json.metadata.Video.video_encoding;
      mediaType = 'video';
    } else if (!!json.metadata.Document) {
      documentData = new DocumentData(
        new Dimensions(
          json.metadata.Document.page_size.width,
          json.metadata.Document.page_size.height,
        ),
        json.metadata.Document.pages,
        json.metadata.Document.author,
        json.metadata.Document.title
      );

      mediaType = 'document';
    }

    let token = '';
    if (auth.token) {
      token = `token=${encodeURIComponent(auth.token)}`;
    }

    return new Media(
      json.id,
      videoEncoding,
      dimensions,
      duration,
      json.file_size,
      json.sha1,
      json.sha256,
      json.md5,
      json.mime,
      documentData,
      mediaType,
      `${environment.api}/buckets/${auth.bucketId}/media/${json.id}/file?${token}`
    );
  }

  private mapPostDetail(json: any): PostDetail {
    return new PostDetail(
      json.post.id,
      json.post.source,
      json.post.title,
      json.post.description,
      new Date(json.post.created_at),
      json.item_count,
      json.tags.map((x: any) => this.mapTag(x))
    )
  }

  private mapTag(json: any): Tag {
    return new Tag(json.id, json.name, null);
  }

  private mapPost(json: any): Post {
    return new Post(
      json.id,
      json.source,
      json.title,
      json.description,
      new Date(json.created_at),
    )
  }

  private mapSearchPostItem(auth: Auth, json: any): SearchPostItem {
    return new SearchPostItem(
      json.item.post.id,
      json.item.position,
      this.mapMedia(auth, json.thumbnail)
    );
  }

  private mapPostItem(auth: Auth, json: any): PostItemDetail {
    return new PostItemDetail(
      json.post.obj.id,
      json.position,
      this.mapMedia(auth, json.content.obj.content.obj),
    );
  }
}
