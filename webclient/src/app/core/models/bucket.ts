import {Auth} from "@core/models/auth";

export class Bucket {
  public constructor(private _id: number,
                     private _name: string,
                     private _passwordProtected: boolean,
                     private _encrypted: boolean) {
  }

  get id(): number {
    return this._id;
  }

  get name(): string {
    return this._name;
  }

  get passwordProtected(): boolean {
    return this._passwordProtected;
  }

  get encrypted(): boolean {
    return this._encrypted;
  }
}

export interface AuthenticatedBucket {
  bucket: Bucket;
  auth: Auth | null;
}

export interface SelectedBucket {
  bucket: Bucket;
  auth: Auth;
}

export class BucketDetails {
  public constructor(private _totalFileSize: number, private _fileCount: number, private _sessionsCreated: number) {
  }

  get totalFileSize(): number {
    return this._totalFileSize;
  }

  get fileCount(): number {
    return this._fileCount;
  }

  get sessionsCreated(): number {
    return this._sessionsCreated;
  }
}
