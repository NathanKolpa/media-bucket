import {Component} from '@angular/core';
import {fromSearch, searchActions} from '@features/search/store';
import {Store} from "@ngrx/store";
import {UploadJob} from "@core/models";
import {fromBucket} from '@features/bucket/store';
import {UploadPositionSwapEvent} from "@features/search/components/upload-list/upload-list.component";

@Component({
  selector: 'app-upload-progress-dialog',
  templateUrl: './upload-progress-dialog.component.html',
  styleUrls: ['./upload-progress-dialog.component.scss']
})
export class UploadProgressDialogComponent {

  jobs$ = this.store.select(fromSearch.selectCurrentJobs);
  bucket$ = this.store.select(fromBucket.selectBucket);

  constructor(private store: Store) {
  }

  trackBy(index: number, job: UploadJob): string {
    return job.id
  }

  swapUploads(jobId: string, event: UploadPositionSwapEvent) {
    this.store.dispatch(searchActions.swapUpload({jobId, aIndex: event.aIndex, bIndex: event.bIndex}));
  }

  deleteUploads(jobId: string, indexes: number[]) {
    this.store.dispatch(searchActions.deleteUploads({jobId, indexes}));

  }
}
