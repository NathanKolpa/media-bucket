export class Page {
  constructor(private _params: PageParams, private _totalRows: number) {
  }

  get totalRows(): number {
    return this._totalRows;
  }

  get params(): PageParams {
    return this._params;
  }

  public nextPage(): PageParams {
    return new PageParams(this.params.pageSize, this.params.offset + this.params.pageSize);
  }
}

export class PageParams {
  constructor(private _pageSize: number, private _offset: number) {
  }

  get pageSize(): number {
    return this._pageSize;
  }

  get offset(): number {
    return this._offset;
  }
}
