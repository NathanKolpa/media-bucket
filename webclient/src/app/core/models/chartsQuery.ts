import {PostSearchQuery} from "./searchQuery";

export type ChartDiscriminator = 'none' | 'duration';
export type ChartSelect = 'count';

export class ChartsQuery {
  public constructor(private _name: string, private _filter: PostSearchQuery, private _select: ChartSelect, private _discriminator: ChartDiscriminator, private _duration: number | null) {
  }

  get name(): string {
    return this._name;
  }

  get filter(): PostSearchQuery {
    return this._filter;
  }

  get discriminator(): ChartDiscriminator {
    return this._discriminator;
  }

  get duration(): number | null {
    return this._duration;
  }


  get select(): ChartSelect {
    return this._select;
  }
}
