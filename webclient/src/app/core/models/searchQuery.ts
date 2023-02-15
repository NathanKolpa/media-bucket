import {Tag} from "./tag";

export type SearchQueryItem =
  { type: 'tag', tag: Tag };

export class PostSearchQuery {

  public static empty(): PostSearchQuery {
    return new PostSearchQuery([]);
  }

  constructor(private _items: SearchQueryItem[]) {
  }

  get items(): SearchQueryItem[] {
    return this._items;
  }

  public addTag(tag: Tag): PostSearchQuery {
    return new PostSearchQuery([...this._items.filter(x => x.tag?.id != tag.id), {type: 'tag', tag}]);
  }

  public removeItem(index: number): PostSearchQuery {
    let copy = [...this._items];
    copy.splice(index, 1);
    return new PostSearchQuery(copy);
  }
}
