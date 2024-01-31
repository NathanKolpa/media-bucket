import { Injectable } from "@angular/core";
import { Actions, createEffect, ofType } from "@ngrx/effects";
import { ApiService } from "@core/services";
import * as bucketActions from './bucket.actions';
import { catchError, map, switchMap, tap } from "rxjs";
import { authActions } from "@core/store/auth";
import { MatDialog } from "@angular/material/dialog";
import {
  BucketDetailsDialogComponent
} from "@features/bucket/containers/bucket-details-dialog/bucket-details-dialog.component";
import { ReloginModalComponent } from "../containers/relogin-modal/relogin-modal.component";

@Injectable()
export class BucketEffects {

  loadBucket$ = createEffect(() => this.actions$.pipe(
    ofType(bucketActions.loadBucket),
    switchMap(({ id }) => this.api.getBucketById(id).pipe(
      map(bucket => bucketActions.loadBucketSuccess({ bucket })),
      catchError(async failure => bucketActions.loadBucketFailure({ failure })),
    ))
  ));

  $logout = createEffect(() => this.actions$.pipe(
    ofType(bucketActions.logout),
    map(({ auth }) => authActions.logout({ auth }))
  ));

  $showGeneralInfo = createEffect(() => this.actions$.pipe(
    ofType(bucketActions.showGeneralInfo),
    tap(() => {
      this.dialog.open(BucketDetailsDialogComponent);
    }),
    map(({ auth }) => bucketActions.loadBucketDetails({ auth }))
  ));

  loadBucketDetails$ = createEffect(() => this.actions$.pipe(
    ofType(bucketActions.loadBucketDetails),
    switchMap(({ auth }) => this.api.getBucketDetails(auth).pipe(
      map(details => bucketActions.loadBucketDetailsSuccess({ details })),
      catchError(async failure => bucketActions.loadBucketDetailsFailure({ failure })),
    ))
  ));

  showRelogin$ = createEffect(() => this.actions$.pipe(
    ofType(authActions.failedAuth),
    tap(({ failure }) => {
      this.dialog.open(ReloginModalComponent, { data: { failure }, disableClose: true })
    })
  ), { dispatch: false })

  relogin$ = createEffect(() => this.actions$.pipe(
    ofType(bucketActions.relogin),
    switchMap(({ bucket, oldAuth, password }) => this.api.login(bucket.id, password, oldAuth?.privateSession ?? true).pipe(
      map(auth => bucketActions.reloginSuccess({ bucket, auth })),
      catchError(async failure => bucketActions.reloginFailure({ failure })),
    ))
  ));

  reloginSuccess$ = createEffect(() => this.actions$.pipe(
    ofType(bucketActions.reloginSuccess),
    map(({ auth }) => authActions.addLogin({ auth }))
  ))

  public constructor(private actions$: Actions, private api: ApiService, private dialog: MatDialog) {
  }
}
