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
  constructor(postId: number, position: number, private _thumbnail: Media) {
    super(postId, position);
  }

  get thumbnail(): Media {
    return this._thumbnail;
  }


  get altTitle(): string | null {
    return null;
  }

  get containsDocument(): boolean {
    return false;
  }

  get containsImages(): boolean {
    return false;
  }

  get containsMovingImages(): boolean {
    return false;
  }

  get containsVideos(): boolean {
    return false;
  }

  get duration(): number | null {
    return null;
  }

  get itemCount(): number {
    return 1;
  }

  get title(): string | null {
    return null;
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
