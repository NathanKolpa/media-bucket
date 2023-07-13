import {ChartDiscriminator} from "@core/models/chartsQuery";

export type ChartPoint =
  { type: 'time', y: number, x: Date }
  | { type: 'none', y: number };

export class ChartSeries {
  constructor(private _name: string, private _points: ChartPoint[]) {
  }

  get name(): string {
    return this._name;
  }

  get points(): ChartPoint[] {
    return this._points;
  }
}

export class Chart {
  constructor(private _name: string | null, private _series: ChartSeries[], private _discriminator: ChartDiscriminator) {
  }

  get series(): ChartSeries[] {
    return this._series;
  }

  get discriminator(): ChartDiscriminator {
    return this._discriminator;
  }

  get name(): string | null {
    return this._name;
  }
}
