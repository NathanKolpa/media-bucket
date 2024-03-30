import { Injectable } from "@angular/core";
import { Actions, createEffect, ofType } from "@ngrx/effects";
import { ApiService } from "@core/services";
import * as searchActions from './search.actions';
import {
  auditTime,
  catchError,
  combineLatest,
  debounceTime,
  filter,
  first,
  forkJoin,
  from,
  map,
  mergeMap,
  mergeWith,
  switchMap,
  tap,
  withLatestFrom
} from "rxjs";
import { Store } from "@ngrx/store";
import * as fromSearch from "./search.selectors";
import { MatDialog } from "@angular/material/dialog";
import { PostDetailDialogComponent } from "@features/search/containers/post-detail-dialog/post-detail-dialog.component";
import { UploadDialogComponent } from "@features/search/containers/upload-dialog/upload-dialog.component";
import { MatSnackBar } from "@angular/material/snack-bar";
import {
  UploadProgressDialogComponent
} from "@features/search/containers/upload-progress-dialog/upload-progress-dialog.component";
import { Page, PageParams } from "@core/models";
import {
  ConfirmDeletePostDialogComponent
} from "@features/search/components/confirm-delete-post-dialog/confirm-delete-post-dialog.component";
import { ManageTagsDialogComponent } from "@features/search/containers/manage-tags-dialog/manage-tags-dialog.component";
import { loadNeighbourItemsSuccess } from "./search.actions";

@Injectable()
export class SearchEffects {

  loadNext$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.loadNext, searchActions.searchQueryChange, searchActions.viewTagLinkedPosts, searchActions.addTagToSearchQuery),
    withLatestFrom(this.store.select(fromSearch.selectNextPage)),
    withLatestFrom(this.store.select(fromSearch.selectSearchQuery)),
    switchMap(([[action, page], query]) => this.api.searchPosts(action.bucket.auth, query, page).pipe(
      map(({ page, posts }) => searchActions.loadNextSuccess({ page, posts })),
      catchError(async failure => searchActions.loadNextFailure({ failure }))
    ))
  ));


  loadNextPostItems$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.loadNextPostItemList),
    withLatestFrom(this.store.select(fromSearch.selectNextItemPage)),
    switchMap(([action, page]) => this.api.searchPostItems(action.bucket.auth, action.postId, page).pipe(
      map(({ page, items }) => searchActions.loadNextPostItemListSuccess({ items })),
      catchError(async failure => searchActions.loadNextPostItemListFailure({ failure }))
    ))
  ));

  loadNextLoadTagEditNextSearchTags$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.loadTagEditNextSearchTags, searchActions.tagEditSearchQueryChange),
    withLatestFrom(this.store.select(fromSearch.selectNextTagEditSearchPage)),
    withLatestFrom(this.store.select(fromSearch.selectTagEditSearchText)),
    auditTime(250),
    switchMap(([[action, page], query]) => this.api.searchTags(action.bucket.auth, page, query || "").pipe(
      map(({ page, tags }) => searchActions.loadTagEditNextSearchTagsSuccess({ page, tags })),
      catchError(async failure => searchActions.loadTagEditNextSearchTagsFailure({ failure }))
    ))
  ));

  searchTextChange$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.searchTextChange),
    auditTime(250),
    switchMap(({ bucket, query }) => this.api.searchTags(bucket.auth, new PageParams(25, 0), query ?? '').pipe(
      map(({ page, tags }) => searchActions.searchTagSuccess({ tags })),
      catchError(async failure => searchActions.searchTagFailure({ failure }))
    ))
  ));

  showPostLoadItem$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.showPost),
    withLatestFrom(this.store.select(fromSearch.selectCurrentItem)),
    switchMap(([{ bucket, postId }, currentItem]) => [
      searchActions.loadPostItem({ bucket, postId, position: currentItem?.position ?? 0 }),
    ])
  ));


  showPostByOffset$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.showPostByOffset),
    switchMap(action => {
      return this.store.select(fromSearch.selectPosts).pipe(

        filter(x => x.length > action.offset),
        first(),
        map(posts => {
          return ({ posts, action });
        })
      )
    }),
    map(({ action, posts }) => searchActions.showPost({ bucket: action.bucket, postId: posts[action.offset].id, showPopup: action.showPopup, offset: action.offset }))
  ));

  showPost$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.showPost),
    tap(({ showPopup }) => {
      if (showPopup) {
        this.dialog.open(PostDetailDialogComponent, {
          backdropClass: 'backdrop',
          panelClass: 'post-detail-container',
          disableClose: true
        });
      }
    }),
    switchMap(({ postId, bucket }) => this.api.getPostById(bucket.auth, postId).pipe(
      map((post) => searchActions.showPostSuccess({ post })),
      catchError(async failure => searchActions.showPostFailure({ failure }))
    ))
  ));

  showUploadDialog$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.showUploadDialog),
    tap(() => {
      this.dialog.open(UploadDialogComponent);
    })
  ), { dispatch: false })

  showUploadProgressDialog$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.showUploadProgress),
    tap(() => {
      this.dialog.open(UploadProgressDialogComponent);
    })
  ), { dispatch: false })

  loadPostDetail$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.showPostSidebar),
    switchMap(({ postId, bucket }) => this.api.getPostById(bucket.auth, postId).pipe(
      map((post) => searchActions.showPostSidebarSuccess({ post })),
      catchError(async failure => searchActions.showPostSidebarFailure({ failure }))
    ))
  ));

  loadTagDetail$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.tagEditSelectTag),
    switchMap(({ tagId, bucket }) => this.api.getTagById(bucket.auth, tagId).pipe(
      map((tag) => searchActions.tagEditSelectTagSuccess({ tag })),
      catchError(async failure => searchActions.tagEditSelectTagFailure({ failure }))
    ))
  ));

  refreshLoaded$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.refreshLoaded),
    withLatestFrom(this.store.select(fromSearch.selectTotalPosts)),
    withLatestFrom(this.store.select(fromSearch.selectNextPage)),
    withLatestFrom(this.store.select(fromSearch.selectSearchQuery)),
    switchMap(([[[{ bucket }, postCount], nextPage], query]) => {
      let chunkCount = postCount / nextPage.pageSize;
      let requests = [];

      for (let i = 0; i < chunkCount; i++) {
        requests.push(this.api.searchPosts(bucket.auth, query, new PageParams(nextPage.pageSize, i)))
      }

      return forkJoin(requests);
    }),
    map((responses) => searchActions.refreshLoadedSuccess({ posts: responses.map(x => x.posts).reduce((acc, posts) => [...acc, ...posts], []) })),
    catchError(async failure => searchActions.refreshLoadedFailure({ failure }))
  ))

  loadPostItem$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.loadPostItem),
    switchMap(({ postId, bucket, position }) => this.api.getPostItemById(bucket.auth, postId, position).pipe(
      map((item) => searchActions.loadPostItemSuccess({ item })),
      catchError(async failure => searchActions.loadPostItemFailure({ failure }))
    ))
  ));

  loadPostItemNeighbours$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.loadPostItem),
    switchMap(({ postId, bucket, position }) => {
      let startPosition = Math.max(position - 1, 0);
      return this.api.searchPostItems(bucket.auth, postId, new PageParams(Math.min(position + 2, 3), startPosition)).pipe(
        map(({ items, page }) => searchActions.loadNeighbourItemsSuccess({ items, startPosition, position, total: page.totalRows })),
        catchError(async failure => searchActions.loadNeighbourItemsFailure({ failure }))
      );
    })
  ));

  openManageTags$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.openManageTags),
    tap(() => {
      this.dialog.open(ManageTagsDialogComponent);
    }),
  ), { dispatch: false });


  requestPostDelete$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.requestPostDelete),
    switchMap(({ post, bucket }) => this.dialog.open(ConfirmDeletePostDialogComponent, { data: post }).afterClosed().pipe(
      filter(x => x === true),
      map(() => searchActions.deletePost({ bucket, postId: post.id }))
    ))
  ));

  updatePost$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.updatePost),
    switchMap(({
      postId,
      source,
      description,
      title,
      tags,
      bucket
    }) => this.api.updatePost(bucket.auth, postId, title, description, source, tags.map(x => x.id)).pipe(
      switchMap((post) => this.api.getPostById(bucket.auth, post.id).pipe(
        map((detail) => searchActions.updatePostSuccess({ post, tags, detail })),
      )),
      catchError(async failure => searchActions.updatePostFailure({ failure }))
    ))
  ));

  updatePostSuccess$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.updatePostSuccess),
    tap(() => {
      this.snackBar.open('Successfully updated!', undefined, {
        duration: 3000
      });
    })
  ), { dispatch: false });

  deletePost$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.deletePost),
    switchMap(({ postId, bucket }) => this.api.deletePost(bucket.auth, postId).pipe(
      map(() => searchActions.deletePostSuccess({ postId })),
      catchError(async failure => searchActions.deletePostFailure({ failure }))
    ))
  ));

  deletePostSuccess$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.deletePostSuccess),
    tap(() => {
      this.snackBar.open('Successfully deleted!', undefined, {
        duration: 3000
      });
    })
  ), { dispatch: false });

  startUploadJob$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.startUploadJob),
    tap(x => {
      let snackBar = this.snackBar.open(`Started uploading ${x.job.nonDeletedUploads.length} files(s)`, 'View', {
        duration: 3000,
      });

      snackBar.onAction().subscribe(() => this.store.dispatch(searchActions.showUploadProgress()));
    }),

    mergeMap(({ job, bucket }) => {
      let cancellation = this.actions$.pipe(
        ofType(searchActions.reset)
      );

      let uploadJobs = from(job.nonDeletedUploads.map((x, i) => {
        let specificUploadCancellation = this.actions$.pipe(
          ofType(searchActions.deleteUploads),
          filter(({ jobId, indexes }) => jobId == job.id && indexes.find(x => x == i) !== undefined)
        )

        return {
          upload: x,
          index: i,
          cancellation: cancellation.pipe(mergeWith(specificUploadCancellation))
        };
      }))
        .pipe(
          mergeMap(({ upload, index, cancellation }) => this.api.uploadFile(bucket.auth, upload.file, cancellation).pipe(
            tap((uploadEvent) => {
              if (uploadEvent.type == 'progress') {
                this.store.dispatch(searchActions.uploadProgress({
                  jobId: job.id,
                  index,
                  uploadedBytes: uploadEvent.uploadedBytes!
                }));
              } else if (uploadEvent.type == 'complete') {
                this.store.dispatch(searchActions.uploadDone({
                  jobId: job.id,
                  index,
                  content: uploadEvent.content!,
                  thumbnail: uploadEvent.thumbnail!
                }));
              }
            }),
            catchError(async failure => {
              this.store.dispatch(searchActions.uploadFailure({ jobId: job.id, index, failure }));
            }),
          ))
        )

      return combineLatest([this.store.select(fromSearch.selectJob(job.id)), uploadJobs]).pipe(
        filter(([job]) => !!job?.successFullyUploaded),
        first(),
        switchMap(([job]) => {
          if (job == null) {
            throw new Error('Job null');
          }

          let uploads = job.nonDeletedSortedUploads;

          if (job.type == 'createPost') {
            return this.api.createPost(bucket.auth, job.createPostData, uploads).pipe(
              map(posts => searchActions.uploadJobPostCreatedSuccess({
                bucket,
                jobId: job.id,
                posts: posts.posts,
                batchId: posts.batchId
              }))
            )
          } else {
            throw new Error('Job type not supported');
          }
        }),
        catchError(async failure => searchActions.uploadJobFailure({ jobId: job.id, failure }))
      );

    })
  ));

  uploadJobPostCreatedSuccess$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.uploadJobPostCreatedSuccess),
    tap(({ posts }) => {
      let snackBar = this.snackBar.open(`Successfully created ${posts.length} posts(s)`, 'Show all', {
        duration: 3000,
      });
    }),
    map(({ bucket }) => searchActions.refreshLoaded({ bucket }))
  ))

  editTag$ = createEffect(() => this.actions$.pipe(
    ofType(searchActions.editTag),
    switchMap(({ bucket, tagId }) => [
      searchActions.openManageTags({ bucket }),
      searchActions.tagEditSelectTag({ bucket, tagId })
    ])
  ));

  public constructor(
    private snackBar: MatSnackBar,
    private dialog: MatDialog,
    private actions$: Actions,
    private api: ApiService,
    private store: Store) {
  }
}
