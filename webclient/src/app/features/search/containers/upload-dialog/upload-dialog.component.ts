import {Component, OnDestroy, OnInit} from '@angular/core';
import {CreatePostData, SelectedBucket, Tag, Upload, UploadJob} from "@core/models";
import {UploadPositionSwapEvent} from "@features/search/components/upload-list/upload-list.component";
import {AppTitleService} from "@core/services";
import {Store} from "@ngrx/store";
import {fromBucket} from '@features/bucket/store';
import {searchActions} from '@features/search/store';
import {MatDialogRef} from "@angular/material/dialog";

function emptyJob(): UploadJob {
  return UploadJob.newPostUpload([], new CreatePostData(null, null, null, false, []));;
}

@Component({
  selector: 'app-upload-dialog',
  templateUrl: './upload-dialog.component.html',
  styleUrls: ['./upload-dialog.component.scss']
})
export class UploadDialogComponent implements OnInit, OnDestroy {

  bucket$ = this.store.select(fromBucket.selectBucket);

  public title: string | null = null;
  public source: string | null = null;
  public description: string | null = null;
  public flatten: boolean = false;
  public createAnother = false;
  public tags: Tag[] = [];

  job: UploadJob = emptyJob();

  constructor(private titleService: AppTitleService, private store: Store, private dialogRef: MatDialogRef<UploadDialogComponent>) {
  }

  ngOnInit(): void {
    this.titleService.push('Upload');
  }

  ngOnDestroy(): void {
    this.titleService.pop();
  }

  addFiles(files: File[]) {
    this.job = this.job.addFiles(files);
  }

  swapUploads(event: UploadPositionSwapEvent) {
    this.job = this.job.moveUploadToIndex(event.aIndex, event.bIndex);
  }

  deleteUploads(indexes: number[]) {
    this.job = this.job.deleteUploads(indexes);
  }

  submit(bucket: SelectedBucket) {
    let job = this.job.setPostData(this.job.createPostData
      .setSource(this.source)
      .setTitle(this.title)
      .setDescription(this.description)
      .setFlatten(this.flatten)
      .setTags(this.tags)
    ).normalize();

    this.store.dispatch(searchActions.startUploadJob({ bucket, job }));

    this.job = emptyJob();

    if (!this.createAnother) {
      this.dialogRef.close();
    }
  }
}
