export class Auth {
  public constructor(private _bucketId: number, private _token: string | null, private _privateSession: boolean, private _domain: string, private _path: string, private _protocol: string, private _port: string | null) {
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

  get domain(): string {
    return this._domain;
  }

  get path(): string {
    return this._path;
  }

  get protocol(): string {
    return this._protocol;
  }

  get port(): string | null {
    return this._port;
  }

  get base(): string {
    return `${this.protocol}//${this.domain}${this.port == null ? '' : ':' + this.port}${this.path}`
  }
}
