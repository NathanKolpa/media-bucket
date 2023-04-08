import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output, ViewChild} from '@angular/core';
import {Upload} from "@core/models";
import {CdkDragDrop} from "@angular/cdk/drag-drop";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";

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
export class UploadListComponent {

  @ViewChild('viewport', {static: true})
  public viewport!: CdkVirtualScrollViewport;

  public sortedUploads: UploadListItem[] = [];

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

  drop(event: CdkDragDrop<Upload[]>) {
    this.swapUploads.emit({
      aIndex: this.sortedUploads[event.previousIndex].index,
      bIndex: this.sortedUploads[event.currentIndex].index
    });
  }

  trackBy(item: any): number {
    return item.index;
  }
}
