import { ChangeDetectionStrategy, Component } from '@angular/core';
import { bucketActions, fromBucket } from '@features/bucket/store';
import { Store } from "@ngrx/store";
import { Auth } from "@core/models";
import { environment } from "@src/environments/environment";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-bucket-details-dialog',
  templateUrl: './bucket-details-dialog.component.html',
  styleUrls: ['./bucket-details-dialog.component.scss']
})
export class BucketDetailsDialogComponent {
  bucket$ = this.store.select(fromBucket.selectBucket);
  bucketLoadingState$ = this.store.select(fromBucket.selectBucketLoadingState);
  detailsLoadingState$ = this.store.select(fromBucket.selectDetailsLoadingState);
  details$ = this.store.select(fromBucket.selectDetails);

  reloadDetails(auth: Auth) {
    this.store.dispatch(bucketActions.loadBucketDetails({ auth }));
  }

  constructor(private store: Store) {
  }


  private apiUrlStr = environment.api;

  public get apiUrl(): string {
    if (new URL(document.baseURI).origin !== new URL(this.apiUrlStr, document.baseURI).origin) {
      return this.apiUrlStr;
    }

    return (new URL(document.baseURI)).toString().replace(/\/+$/, "") + this.apiUrlStr;
  }
}
