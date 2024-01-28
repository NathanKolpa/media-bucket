import {ChangeDetectionStrategy, Component, EventEmitter, Input, OnDestroy, Output, ViewChild} from '@angular/core';
import {LoadingState} from "@core/models";
import {BreakpointObserver, Breakpoints} from "@angular/cdk/layout";
import {Subscription} from "rxjs";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";
import {Listing} from "@core/models/listing";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-search-results',
  templateUrl: './search-results.component.html',
  styleUrls: ['./search-results.component.scss']
})
export class SearchResultsComponent implements OnDestroy {
  private _startingIndex: number | null = null;

  @ViewChild('viewport', {static: true})
  viewport!: CdkVirtualScrollViewport;

  @Output()
  public requestNextData = new EventEmitter();

  @Output()
  public showInfo = new EventEmitter<number>();

  @Output()
  public showDetail = new EventEmitter<number>();

  @Input()
  public disableFooter = false;

  @Input()
  set startingIndex(value: number | null) {
    if (value === null) {
      return;
    }

    this._startingIndex = value;
    this.moveToStartingPosition()
  }


  public rows: number[][] = [];
  @Input()
  public nextLoadingState: LoadingState | null = null;
  @Input()
  public resultCount: number | null = null;
  private requestedData: boolean = false;
  private breakpointSubscription: Subscription;

  constructor(private breakpointObserver: BreakpointObserver) {

    this.breakpointSubscription = this.breakpointObserver.observe([
      Breakpoints.XSmall,
      Breakpoints.Small,
      Breakpoints.Medium,
      Breakpoints.Large,
      Breakpoints.XLarge,
    ])
      .subscribe(() => {
        if (this.breakpointObserver.isMatched(Breakpoints.XSmall)) {
          this.rowsSize = 1;
        } else if (this.breakpointObserver.isMatched(Breakpoints.Small)) {
          this.rowsSize = 2;
        } else if (this.breakpointObserver.isMatched(Breakpoints.Medium)) {
          this.rowsSize = 3;
        } else {
          this.rowsSize = 5;
        }
      });
  }

  private _items: Listing[] = [];

  get items(): Listing[] {
    return this._items;
  }

  @Input()
  set items(value: Listing[]) {
    this._items = value;
    this.requestedData = false;
    this.generateRows();

    if (this.viewport) {
      this.viewport.checkViewportSize();
      this.moveToStartingPosition();
    }
  }

  private _rowsSize = 3;

  get rowsSize(): number {
    return this._rowsSize;
  }

  set rowsSize(value: number) {
    this._rowsSize = value;
    this.generateRows();
  }

  public get containerClass(): string {
    return `row-cols-${this._rowsSize}`;
  }

  ngOnDestroy(): void {
    this.breakpointSubscription.unsubscribe();
  }

  trackById(i: number) {
    return i;
  }

  scrolledIndexChange() {
    if (this.requestedData) {
      return;
    }

    let end = this.viewport.getRenderedRange().end;

    if (end >= this.rows.length - 1 && this._items.length > 0) {
      this.requestNextData.emit();
      this.requestedData = true;
    }
  }

  private moveToStartingPosition() {
    if (!this.viewport || this._startingIndex == null) {
      return;
    }

    const startingRow = Math.floor(this._startingIndex / this._rowsSize);

    if (this.rows.length >= startingRow && startingRow > 0) {
      this._startingIndex = null;
      setTimeout(() => this.viewport.scrollToIndex(startingRow), 0);
    }
  }

  private generateRows() {
    let result: number[][] = [];

    for (let i = 0; i < this._items.length; i += this.rowsSize) {
      let row = [];

      for (let y = 0; y < this.rowsSize; y++) {
        row.push(i + y);
      }

      result.push(row);
    }

    this.rows = result;
  }
}
