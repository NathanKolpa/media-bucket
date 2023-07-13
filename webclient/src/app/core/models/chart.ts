export type ChartPoint =
  { type: 'time', y: number, x: Date }
  | { type: 'none', y: number };

export class Chart {
  constructor(private _name: string, private _points: ChartPoint[]) {
  }

  get name(): string {
    return this._name;
  }

  get points(): ChartPoint[] {
    return this._points;
  }
}
