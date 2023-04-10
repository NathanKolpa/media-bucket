import {
  AfterViewInit,
  ChangeDetectionStrategy,
  Component,
  EventEmitter,
  Input,
  Output,
  ViewChild,
  ViewEncapsulation
} from '@angular/core';
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

  private range: ListRange | null = null;
  private viewportSub: Subscription | null = null;

  constructor() {

  }

  ngAfterViewInit(): void {
    this.viewportSub = this.viewport.viewChange.subscribe(x => {
      this.range = x;
    });
  }

  @Input()
  public set uploads(value: Upload[]) {
    let copy = value
      .map((upload, index) => ({upload, index}))
      .filter(x => x.upload.state !== 'deleted');
    copy.sort((a, b) => a.upload.position - b.upload.position);
    this.sortedUploads = copy;
  }

  @Output()
  public swapUploads = new EventEmitter<UploadPositionSwapEvent>();

  @Output()
  public deleteIndexes = new EventEmitter<number[]>();

  drop(event: CdkDragDrop<number>) {
    if (this.range == null) {
      return;
    }

    this.swapUploads.emit({
      aIndex: this.sortedUploads[event.item.data as number].index,
      bIndex: this.sortedUploads[event.currentIndex + this.range.start].index
    });
  }

  trackBy(item: any): number {
    return item.index;
  }
}
