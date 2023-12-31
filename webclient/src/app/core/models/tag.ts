import {group} from "@angular/animations";

export class Tag {
  constructor(private _id: number, private _name: string, private _group: TagGroup | null, private _linkedPosts: number | null, private _createdAt: Date) {
  }

  get id(): number {
    return this._id;
  }

  get name(): string {
    return this._name;
  }

  get group(): TagGroup | null {
    return this._group;
  }

  get linkedPosts(): number | null {
    return this._linkedPosts;
  }

  get createdAt(): Date {
    return this._createdAt;
  }
}

export class TagGroup {
  constructor(private _id: number, private _name: string, private _color: string) {
  }

  get id(): number {
    return this._id;
  }

  get name(): string {
    return this._name;
  }

  get color(): string {
    return this._color;
  }
}

export class TagDetail extends Tag {

  constructor(id: number, name: string, group: TagGroup | null, linkedPosts: number | null, createdAt: Date, private _implies: Tag[]) {
    super(id, name, group, linkedPosts, createdAt);
  }
}
