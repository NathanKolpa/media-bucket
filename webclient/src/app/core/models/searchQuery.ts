import {Tag} from "./tag";

export type SearchQueryItem =
  { type: 'tag', tag: Tag }
  | {type: 'text', str: string};

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
    return new PostSearchQuery([...this._items.filter(x => {
      if (x.type != 'tag') {
        return true;
      }

      return  x.tag.id != tag.id;
    }), {type: 'tag', tag}]);
  }

  public addText(text: string): PostSearchQuery {
    return new PostSearchQuery([...this._items.filter(x => {
      if (x.type != 'text') {
        return true;
      }

      return  x.str != text;
    }), {type: 'text', str: text}]);
  }

  public removeItem(index: number): PostSearchQuery {
    let copy = [...this._items];
    copy.splice(index, 1);
    return new PostSearchQuery(copy);
  }
}
