import {Media} from "./media";
import {Tag} from "@core/models/tag";

export class Post {
  constructor(
    private _id: number,
    private _source: string | null,
    private _title: string | null,
    private _description: string | null,
    private _createdAt: Date) {
  }

  get id(): number {
    return this._id;
  }

  get source(): string | null {
    return this._source;
  }

  get title(): string | null {
    return this._title;
  }

  get description(): string | null {
    return this._description;
  }

  get createdAt(): Date {
    return this._createdAt;
  }
}

export class SearchPost extends Post {
  constructor(
    id: number,
    source: string | null,
    title: string | null,
    description: string | null,
    createdAt: Date,
    private _itemCount: number,
    private _containsImages: boolean,
    private _containsVideos: boolean,
    private _containsMovingImages: boolean,
    private _duration: number | null,
    private _thumbnail: Media | null) {
    super(id, source, title, description, createdAt);
  }

  get itemCount(): number {
    return this._itemCount;
  }

  get containsImages(): boolean {
    return this._containsImages;
  }

  get containsVideos(): boolean {
    return this._containsVideos;
  }

  get containsMovingImages(): boolean {
    return this._containsMovingImages;
  }

  get duration(): number | null {
    return this._duration;
  }

  get thumbnail(): Media | null {
    return this._thumbnail;
  }
}

export class PostDetail extends Post {
  constructor(id: number, source: string | null, title: string | null, description: string | null, createdAt: Date, private _itemCount: number, private _tags: Tag[]) {
    super(id, source, title, description, createdAt);
  }

  get itemCount(): number {
    return this._itemCount;
  }


  get tags(): Tag[] {
    return this._tags;
  }
}
