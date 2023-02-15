export class Auth {
  public constructor(private _bucketId: number, private _token: string | null, private _privateSession: boolean) {
  }

  get bucketId(): number {
    return this._bucketId;
  }

  get token(): string | null {
    return this._token;
  }

  get privateSession(): boolean {
    return this._privateSession;
  }
}
