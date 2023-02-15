import {Media} from "./media";

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

export class SearchPostItem extends PostItem {
  constructor(postId: number, position: number, private _thumbnail: Media) {
    super(postId, position);
  }

  get thumbnail(): Media {
    return this._thumbnail;
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
