import {Tag} from "./tag";

export type SearchQueryItem =
  { type: 'tag', tag: Tag };

export class PostSearchQuery {

  public static empty(): PostSearchQuery {
    return new PostSearchQuery([], null);
  }

  constructor(private _items: SearchQueryItem[], private _text: string | null) {
  }

  get items(): SearchQueryItem[] {
    return this._items;
  }

  public addTag(tag: Tag): PostSearchQuery {
    return new PostSearchQuery([...this._items.filter(x => x.tag?.id != tag.id), {type: 'tag', tag}], this._text);
  }

  public removeItem(index: number): PostSearchQuery {
    let copy = [...this._items];
    copy.splice(index, 1);
    return new PostSearchQuery(copy, this._text);
  }

  public setText(text: string | null): PostSearchQuery {
    return new PostSearchQuery(this._items, text);
  }

  get text(): string | null {
    return this._text;
  }
}
