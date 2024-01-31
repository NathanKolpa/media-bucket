import { Component, Inject, OnDestroy } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { Bucket, Failure, SelectedBucket } from '@core/models';
import { bucketActions, fromBucket } from '@features/bucket/store';
import { Actions, ofType } from '@ngrx/effects';
import { Store } from '@ngrx/store';
import { Login } from '@shared/login-form/components/login-form/login-form.component';
import { first, map, Subscription } from 'rxjs';

@Component({
  selector: 'app-relogin-modal',
  templateUrl: './relogin-modal.component.html',
  styleUrls: ['./relogin-modal.component.scss']
})
export class ReloginModalComponent implements OnDestroy {

  selectedBucket$ = this.store.select(fromBucket.selectBucket);
  loadingState$ = this.store.select(fromBucket.selectReloginLoadingState);

  private successSub: Subscription;

  constructor(private store: Store, private dialogRef: MatDialogRef<ReloginModalComponent>, action: Actions, @Inject(MAT_DIALOG_DATA) public data: { failure: Failure }) {
    this.successSub = action.pipe(ofType(bucketActions.reloginSuccess), first()).subscribe(() => {
      this.dialogRef.close();
    })
  }

  ngOnDestroy(): void {
    this.successSub.unsubscribe();
  }

  login(bucket: SelectedBucket, req: Login) {
    this.store.dispatch(bucketActions.relogin({ bucket: bucket.bucket, oldAuth: bucket.auth, password: req.password }));
  }
}
