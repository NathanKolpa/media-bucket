import { Tag } from "./tag";

export type SearchQueryItem =
  { type: 'tag', tag: Tag }
  | { type: 'text', str: string };

export type SearchQueryOrder = 'newest' | 'oldest' | 'relevant' | 'random';

export class PostSearchQuery {

  constructor(private _items: SearchQueryItem[], private _order: SearchQueryOrder, private _seed: number) {
  }

  get items(): SearchQueryItem[] {
    return this._items;
  }

  get order(): SearchQueryOrder {
    return this._order;
  }

  get seed(): number {
    return this._seed;
  }

  public static empty(): PostSearchQuery {
    return new PostSearchQuery([], 'relevant', Math.random());
  }

  public randomizeSeed(): PostSearchQuery {
    return new PostSearchQuery(this._items, this._order, Math.random());
  }

  public setTag(tag: Tag): PostSearchQuery {
    return new PostSearchQuery([{ tag, type: 'tag' }], this._order, this._seed);
  }

  public addTag(tag: Tag): PostSearchQuery {
    return new PostSearchQuery([...this._items.filter(x => {
      if (x.type != 'tag') {
        return true;
      }

      return x.tag.id != tag.id;
    }), { type: 'tag', tag }], this._order, this._seed);
  }

  public addText(text: string): PostSearchQuery {
    return new PostSearchQuery([...this._items.filter(x => {
      if (x.type != 'text') {
        return true;
      }

      return x.str != text;
    }), { type: 'text', str: text }], this._order, this._seed);
  }

  public setOrder(order: SearchQueryOrder): PostSearchQuery {
    return new PostSearchQuery(this._items, order, this._seed);
  }

  public removeItem(index: number): PostSearchQuery {
    let copy = [...this._items];
    copy.splice(index, 1);
    return new PostSearchQuery(copy, this._order, this._seed);
  }

  public queryParams(): any {
    let params: any = {
      order: this.order,
    };

    if (this.order == 'random') {
      params.seed = this.seed;
    }

    let tagStr = this.items.filter(x => x.type == 'tag').map((x: any) => x.tag.id).join(',');
    if (tagStr != '') {
      params.tags = tagStr;
    }

    let textStr = JSON.stringify(this.items.filter(x => x.type == 'text').map((x: any) => x.str));

    if (textStr != '[]') {
      params.text = textStr;
    }

    return params;
  }
}
