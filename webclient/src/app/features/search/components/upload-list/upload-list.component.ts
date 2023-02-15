import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Upload} from "@core/models";
import {CdkDragDrop} from "@angular/cdk/drag-drop";
import {SelectionModel} from "@angular/cdk/collections";

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

  mapUploadsToIndexes(uploads: any): number[] {
    return uploads.map((x: any) => x.value.index);
  }
}
