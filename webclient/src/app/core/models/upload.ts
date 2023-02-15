import {Failure} from "./failure";
import {Media} from "@core/models/media";
import {Post} from "@core/models/post";
import {Tag} from "@core/models/tag";

export type UploadState = 'waiting' | 'uploading' | 'done' | 'error' | 'deleted';

export class Upload {

  public static fromFile(file: File, position: number) {
    return new Upload(file, 'waiting', 0, null, position, null, null)
  }

  private constructor(private _file: File,
                      private _state: UploadState,
                      private _uploadedBytes: number,
                      private _failure: Failure | null,
                      private _position: number,
                      private _content: Media | null,
                      private _thumbnail: Media | null) {
  }

  get file(): File {
    return this._file;
  }

  get state(): UploadState {
    return this._state;
  }

  get uploadedBytes(): number {
    return this._uploadedBytes;
  }

  get failure(): Failure | null {
    return this._failure;
  }

  get content(): Media | null {
    return this._content;
  }

  get position(): number {
    return this._position;
  }

  get thumbnail(): Media | null {
    return this._thumbnail;
  }

  public setPosition(position: number): Upload {
    return new Upload(this._file, this._state, this._uploadedBytes, this._failure, position, this._content, this._thumbnail);
  }

  public setProgress(uploadedBytes: number): Upload {
    return new Upload(this._file, 'uploading', uploadedBytes, this._failure, this._position, this._content, this._thumbnail);
  }

  public error(failure: Failure): Upload {
    return new Upload(this._file, 'error', this._uploadedBytes, failure, this._position, this._content, this._thumbnail);
  }

  public done(content: Media, thumbnail: Media): Upload {
    return new Upload(this._file, 'done', this._uploadedBytes, this._failure, this._position, content, thumbnail);
  }

  public delete(): Upload {
    return new Upload(this._file, 'deleted', this._uploadedBytes, this._failure, this._position, this._content, this._thumbnail);
  }

  public get progress(): number {
    return this.uploadedBytes / this.file.size * 100;
  }
}

export type UploadJobType = 'createPost';

export class CreatePostData {
  public constructor(private _title: string | null, private _source: string | null, private _description: string | null, private _flatten: boolean, private _tags: Tag[]) {
  }

  get title(): string | null {
    return this._title;
  }

  get source(): string | null {
    return this._source;
  }

  get description(): string | null {
    return this._description;
  }

  get flatten(): boolean {
    return this._flatten;
  }

  get tags(): Tag[] {
    return this._tags;
  }

  public setTitle(title: string | null): CreatePostData {
    return new CreatePostData(title, this._source, this._description, this._flatten, this._tags);
  }

  public setDescription(description: string | null): CreatePostData {
    return new CreatePostData(this._title, this._source, description, this._flatten, this._tags);
  }

  public setSource(source: string | null): CreatePostData {
    return new CreatePostData(this._title, source, this._description, this._flatten, this._tags);
  }

  public setFlatten(flatten: boolean): CreatePostData {
    return new CreatePostData(this._title, this._source, this._description, flatten, this._tags);
  }

  public setTags(tags: Tag[]): CreatePostData {
    return new CreatePostData(this._title, this._source, this._description, this._flatten, tags);
  }
}

export class UploadJob {
  public static newPostUpload(uploads: Upload[], data: CreatePostData) {
    return new UploadJob(crypto.randomUUID(), uploads, 'createPost', null, data);
  }

  private constructor(private _id: string, private _uploads: Upload[], private _type: UploadJobType, private _failure: Failure | null, private _createPostData: CreatePostData) {
  }

  get uploads(): Upload[] {
    return this._uploads;
  }

  get type(): UploadJobType {
    return this._type;
  }

  get createPostData(): CreatePostData {
    return this._createPostData;
  }

  get id(): string {
    return this._id;
  }

  get failure(): Failure | null {
    return this._failure;
  }

  public updateUpload(index: number, upload: Upload): UploadJob {
    let newUploads = [...this._uploads];
    newUploads[index] = upload;

    return new UploadJob(this._id, newUploads, this._type, this._failure, this._createPostData);
  }

  public setPostData(data: CreatePostData): UploadJob {
    if (this._type != 'createPost') {
      return this;
    }

    return new UploadJob(this._id, this._uploads, this._type, this._failure, data);
  }

  public mapUpload(index: number, mapper: (upload: Upload) => Upload): UploadJob {
    return this.updateUpload(index, mapper(this.uploads[index]));
  }

  public addFiles(files: File[]): UploadJob {
    let newUploads = files.map((x, i) => Upload.fromFile(x, this.uploads.length + i));
    let newList = [...this._uploads, ...newUploads];

    return new UploadJob(this._id, newList, this._type, this._failure, this._createPostData);
  }

  public moveUploadToIndex(uploadIndex: number, targetIndex: number): UploadJob {
    if (uploadIndex == targetIndex) {
      return this;
    }

    let uploadPosition = this._uploads[uploadIndex].position;
    let targetPosition = this._uploads[targetIndex].position;

    let isDeltaUp = targetPosition > uploadPosition;

    let newUploads = this._uploads.map((upload) => {
      let position = upload.position;

      if (uploadPosition == position) {
        return upload.setPosition(targetPosition);
      }

      if (isDeltaUp) {
        if ((position > uploadPosition && position < targetPosition) || position == targetPosition) {
          return upload.setPosition(position - 1);
        }
      }
      else {
        if ((position < uploadPosition && position > targetPosition) || position == targetPosition) {
          return upload.setPosition(position + 1);
        }
      }

      return upload;
    });


    return new UploadJob(this._id, newUploads, this._type, this._failure, this._createPostData);
  }

  public deleteUploads(indexes: number[]): UploadJob {
    let newUploads = this._uploads.map((upload, index) => {
      if (indexes.indexOf(index) !== -1) {
        return upload.delete();
      }

      return upload;
    });

    return new UploadJob(this._id, newUploads, this._type, this._failure, this._createPostData);
  }

  public normalize(): UploadJob {
    return new UploadJob(this._id, this.nonDeletedUploads, this._type, this._failure, this._createPostData);
  }

  public get nonDeletedUploads(): Upload[] {
    return this._uploads.filter(x => x.state !== 'deleted');
  }

  public get nonDeletedSortedUploads(): Upload[] {
    return this.nonDeletedUploads.sort((a, b) => a.position - b.position);
  }

  public isEmpty(): boolean {
    return this.nonDeletedUploads.length <= 0;
  }

  public error(failure: Failure): UploadJob {
    return new UploadJob(this._id, this._uploads, this._type, failure, this._createPostData);
  }

  public postCreated(): UploadJob {
    return new UploadJob(this._id, this._uploads, this._type, this._failure, this._createPostData);
  }

  get successFullyUploaded(): boolean {
    return this.nonDeletedUploads.find(x => x.state != 'done') === undefined;
  }

  get done(): boolean {
    return this.successFullyUploaded && this.failure == null;
  }

  get totalBytes(): number {
    return this.nonDeletedUploads.reduce((acc, u) => acc + u.file.size, 0);
  }

  get uploadedBytes(): number {
    return this.nonDeletedUploads.reduce((acc, u) => acc + u.uploadedBytes, 0);
  }

  get progress(): number {
    return this.uploadedBytes / this.totalBytes * 100;
  }
}
