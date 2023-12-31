import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output, ViewChild} from '@angular/core';
import {LoadingState, Tag, TagDetail} from "@core/models";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-manage-tags-search-results',
  templateUrl: './manage-tags-search-results.component.html',
  styleUrls: ['./manage-tags-search-results.component.scss']
})
export class ManageTagsSearchResultsComponent {

  public displayedColumns = ['name']

  @ViewChild('viewport', {static: true})
  viewport!: CdkVirtualScrollViewport;

  @Input()
  public nextLoadingState: LoadingState | null = null;

  @Output()
  public requestNextData = new EventEmitter();

  @Output()
  public selectTag = new EventEmitter<number | null>();

  @Input()
  public resultCount: number | null = null;

  private _tags: Tag[] = [];

  private requestedData: boolean = false;


  @Input()
  public selectedTag: TagDetail | null = null;

  get tags(): Tag[] {
    return this._tags;
  }

  @Input()
  set tags(value: Tag[]) {
    this._tags = value;
    this.requestedData = false;
  }

  trackBy(item: any): number {
    return item.id;
  }

  scrolledIndexChange() {
    if (this.requestedData) {
      return;
    }

    let end = this.viewport.getRenderedRange().end;

    if (end >= this._tags.length - 1) {
      this.requestNextData.emit();
      this.requestedData = true;
    }
  }
}
