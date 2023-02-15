export class Dimensions {
  constructor(private _width: number, private _height: number) {
  }

  get width(): number {
    return this._width;
  }

  get height(): number {
    return this._height;
  }
}

export class DocumentData {
  constructor(private _pageSize: Dimensions, private _pages: number, private _author: string | null, private _title: string | null) {
  }

  get pageSize(): Dimensions {
    return this._pageSize;
  }

  get pages(): number {
    return this._pages;
  }

  get author(): string | null {
    return this._author;
  }

  get title(): string | null {
    return this._title;
  }
}

export type MediaType = 'unknown' | 'image' | 'video' | 'document';

export class Media {
  constructor(
    private _id: number,
    private _videoEncoding: string | null,
    private _dimensions: Dimensions | null,
    private _duration: number | null,
    private _fileSize: number,
    private _sha1: string,
    private _sha256: string,
    private _md5: string,
    private _mime: string,
    private _documentData: DocumentData | null,
    private _mediaType: MediaType,
    private _url: string
  ) {
  }

  get id(): number {
    return this._id;
  }

  get dimensions(): Dimensions | null {
    return this._dimensions;
  }

  get duration(): number | null {
    return this._duration;
  }

  get fileSize(): number {
    return this._fileSize;
  }

  get sha1(): string {
    return this._sha1;
  }

  get sha256(): string {
    return this._sha256;
  }

  get md5(): string {
    return this._md5;
  }

  get mime(): string {
    return this._mime;
  }

  get documentData(): DocumentData | null {
    return this._documentData;
  }

  get url(): string {
    return this._url;
  }

  get mediaType(): MediaType {
    return this._mediaType;
  }

  get videoEncoding(): string | null {
    return this._videoEncoding;
  }

  public isCompatibleWithBrowser(): boolean {
    let isFirefox = navigator.userAgent.toLowerCase().indexOf('firefox') > -1;

    if (this.mime == 'video/x-matroska' && isFirefox) {
      return false;
    }

    if (this.videoEncoding == 'hevc' && isFirefox) {
      return false;
    }

    if (this.mime == 'application/vnd.openxmlformats-officedocument.wordprocessingml.document') {
      return false;
    }

    return true;
  }
}
