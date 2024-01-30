export class Auth {
  public constructor(private _bucketId: number, private _token: string | null, private _shareToken: string, private _privateSession: boolean, private _domain: string, private _path: string, private _protocol: string, private _port: string | null, private _lifetime: number, private _createdAt: Date) {
  }

  get shareToken(): string {
    return this._shareToken;
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

  public get createdAt(): Date {
    return this._createdAt;
  }

  public get lifetime(): number {
    return this._lifetime;
  }

  public isExpired(): boolean {
    let expireDate = new Date();
    expireDate.setSeconds(this.createdAt.getSeconds() + this.lifetime);
    return new Date() > expireDate;
  }
}
