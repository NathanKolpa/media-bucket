export class Tag {
  constructor(private _id: number, private _name: string, private _group: TagGroup | null) {
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
}

export class TagGroup {
  constructor(private _id: number, private _name: string) {
  }

  get id(): number {
    return this._id;
  }

  get name(): string {
    return this._name;
  }
}
