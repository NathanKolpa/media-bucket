import {AfterViewInit, ChangeDetectionStrategy, Component, EventEmitter, Input, Output, ViewChild} from '@angular/core';
import {Upload} from "@core/models";
import {CdkDragDrop} from "@angular/cdk/drag-drop";
import {CdkVirtualForOf} from "@angular/cdk/scrolling";
import {ListRange} from "@angular/cdk/collections";
import {Subscription} from "rxjs";

export interface UploadPositionSwapEvent {
  aIndex: number,
  bIndex: number
}

interface UploadListItem {
  upload: Upload,
  index: number
}

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-upload-list',
  templateUrl: './upload-list.component.html',
  styleUrls: ['./upload-list.component.scss']
})
export class UploadListComponent implements AfterViewInit {

  @ViewChild(CdkVirtualForOf, {static: true})
  public viewport!: CdkVirtualForOf<UploadListItem[]>;

  public sortedUploads: UploadListItem[] = [];
  @Output()
  public swapUploads = new EventEmitter<UploadPositionSwapEvent>();
  @Output()
  public deleteIndexes = new EventEmitter<number[]>();
  private range: ListRange | null = null;
  private viewportSub: Subscription | null = null;
  private dragItem: number | null = null;
  private hideCompleted = false;

  private original: Upload[] = []

  constructor() {

  }

  @Input()
  public set uploads(value: Upload[]) {
    this.original = value;
    this.updateList();
  }

  updateList() {
    let hideCompleted = this.hideCompleted;

    let copy = this.original
      .map((upload, index) => ({upload, index}))
      .filter(x => x.upload.state !== 'deleted')
      .filter(x => !(hideCompleted && x.upload.state == 'done'));

    copy.sort((a, b) => a.upload.position - b.upload.position);
    this.sortedUploads = copy;
  }

  ngAfterViewInit(): void {
    this.viewportSub = this.viewport.viewChange.subscribe(x => {
      this.range = x;
    });
  }

  public dragStart(index: number) {
    this.dragItem = index;
  }

  drop(event: CdkDragDrop<number>) {
    if (this.range == null || this.dragItem === null) {
      return;
    }

    this.swapUploads.emit({
      aIndex: this.sortedUploads[this.dragItem].index,
      bIndex: this.sortedUploads[event.currentIndex + this.range.start].index
    });
  }

  trackBy(item: any): number {
    return item.index;
  }

  setHideCompleted(value: boolean) {
    this.hideCompleted = value;
    this.updateList();
  }
}
