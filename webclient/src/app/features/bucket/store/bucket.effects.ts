import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {ApiService} from "@core/services";
import * as bucketActions from './bucket.actions';
import {catchError, map, switchMap} from "rxjs";
import {authActions} from "@core/store/auth";

@Injectable()
export class BucketEffects {

  loadBucket$ = createEffect(() => this.actions$.pipe(
    ofType(bucketActions.loadBucket),
    switchMap(({id}) => this.api.getBucketById(id).pipe(
      map(bucket => bucketActions.loadBucketSuccess({bucket})),
      catchError(async failure => bucketActions.loadBucketFailure({failure})),
    ))
  ));

  $logout = createEffect(() => this.actions$.pipe(
    ofType(bucketActions.logout),
    map(({auth}) => authActions.logout({auth}))
  ));

  public constructor(private actions$: Actions, private api: ApiService) {
  }
}
