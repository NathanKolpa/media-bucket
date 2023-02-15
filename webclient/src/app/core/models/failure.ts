export interface Failure {
  readonly message: string
}

export class ApiFailure implements Failure {
  public constructor(private _message: string, private _inner: string, private _status: number, private _statusText: string) {
  }

  get message(): string {
    return this._message;
  }

  get inner(): string {
    return this._inner;
  }

  get status(): number {
    return this._status;
  }

  get statusText(): string {
    return this._statusText;
  }
}
