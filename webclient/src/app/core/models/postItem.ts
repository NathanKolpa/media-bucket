import {Media} from "./media";
import {Listing} from "@core/models/listing";

export class PostItem {
  constructor(private _postId: number, private _position: number) {
  }

  get position(): number {
    return this._position;
  }

  get postId(): number {
    return this._postId;
  }
}

export class SearchPostItem extends PostItem implements Listing {
  constructor(postId: number, position: number, private _containsImage: boolean, private _containsMovingImages: boolean, private _containsVideos: boolean, private _containsDocument: boolean, private _duration: number | null, private _thumbnail: Media) {
    super(postId, position);
  }

  get thumbnail(): Media {
    return this._thumbnail;
  }


  get altTitle(): string | null {
    return null;
  }

  get duration(): number | null {
    return this._duration;
  }

  get itemCount(): number {
    return 1;
  }

  get title(): string | null {
    return null;
  }

  get containsImages(): boolean {
    return this._containsImage;
  }

  get containsMovingImages(): boolean {
    return this._containsMovingImages;
  }

  get containsVideos(): boolean {
    return this._containsVideos;
  }

  get containsDocument(): boolean {
    return this._containsDocument;
  }
}

export class PostItemDetail extends PostItem {
  constructor(postId: number, position: number, private _content: Media) {
    super(postId, position);
  }

  get content(): Media {
    return this._content;
  }
}
