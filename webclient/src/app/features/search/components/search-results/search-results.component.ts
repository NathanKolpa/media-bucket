import {ChangeDetectionStrategy, Component, EventEmitter, Input, OnDestroy, Output, ViewChild} from '@angular/core';
import {LoadingState, SearchPost} from "@core/models";
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

  @ViewChild(CdkVirtualScrollViewport, {static: true})
  viewport!: CdkVirtualScrollViewport;

  @Output()
  public requestNextData = new EventEmitter();

  @Output()
  public showInfo = new EventEmitter<number>();

  @Output()
  public showDetail = new EventEmitter<number>();

  private _items: Listing[] = [];
  private _rowsSize = 3;
  private requestedData: boolean = false;

  public rows: number[][] = [];

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

  @Input()
  set items(value: Listing[]) {
    this._items = value;
    this.requestedData = false;
    this.generateRows();

    if (this.viewport)
      this.viewport.checkViewportSize();
  }

  get items(): Listing[] {
    return this._items;
  }

  @Input()
  public nextLoadingState: LoadingState | null = null;

  @Input()
  public resultCount: number | null = null;

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

  ngOnDestroy(): void {
    this.breakpointSubscription.unsubscribe();
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

  trackById(i: number) {
    return i;
  }

  scrolledIndexChange() {
    if (this.requestedData) {
      return;
    }

    let end = this.viewport.getRenderedRange().end;

    if (end >= this.rows.length - 1) {
      this.requestNextData.emit();
      this.requestedData = true;
    }
  }
}
