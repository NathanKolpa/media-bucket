import {PostSearchQuery} from "./searchQuery";

export type ChartDiscriminatorType = 'none' | 'duration';
export type ChartSelect = 'count';

export type ChartDiscriminatorDuration = 'hour' | 'day' | 'week' | 'month' | 'year';

export class ChartDiscriminator {
  public constructor(private _discriminator: ChartDiscriminatorType, private _duration: ChartDiscriminatorDuration | null) {
  }

  get discriminator(): ChartDiscriminatorType {
    return this._discriminator;
  }

  get duration(): ChartDiscriminatorDuration | null {
    return this._duration;
  }
}

export class ChartSeriesQuery {
  public constructor(private _name: string, private _filter: PostSearchQuery, private _select: ChartSelect) {
  }

  get name(): string {
    return this._name;
  }

  get filter(): PostSearchQuery {
    return this._filter;
  }

  get select(): ChartSelect {
    return this._select;
  }
}

export class ChartQuery {
  public constructor(private _name: string | null, private _series: ChartSeriesQuery[], private _discriminator: ChartDiscriminator) {
  }

  get name(): string | null {
    return this._name;
  }

  get series(): ChartSeriesQuery[] {
    return this._series;
  }

  get discriminator(): ChartDiscriminator {
    return this._discriminator;
  }

  public static initial(): ChartQuery {
    return new ChartQuery(null, [], new ChartDiscriminator('none', null))
  }

  public addSeries(series: ChartSeriesQuery): ChartQuery {
    return new ChartQuery(this._name, [...this._series, series], this._discriminator);
  }

  public removeSeries(index: number): ChartQuery {
    let copy = [...this._series];
    copy.splice(index, 1);
    return new ChartQuery(this._name, copy, this._discriminator);
  }

  public setTitle(title: null | string): ChartQuery {
    return new ChartQuery(title, this._series, this._discriminator);
  }

  public setDiscriminator(value: ChartDiscriminator): ChartQuery {
    return new ChartQuery(this._name, this._series, value);
  }
}

