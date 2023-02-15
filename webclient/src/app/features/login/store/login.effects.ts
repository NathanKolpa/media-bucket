import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {ApiService} from "@core/services";
import * as loginActions from './login.actions';
import {catchError, map, switchMap} from "rxjs";
import {authActions} from "@core/store/auth";

@Injectable()
export class LoginEffects {
  getAllBuckets$ = createEffect(() => this.actions$.pipe(
    ofType(loginActions.getAllBuckets),
    switchMap(() => this.api.getAllBuckets().pipe(
      map((buckets) => loginActions.getAllBucketsSuccess({buckets})),
      catchError(async failure => loginActions.getAllBucketsFailure({failure}))
    ))
  ));

  login$ = createEffect(() => this.actions$.pipe(
    ofType(loginActions.login),
    switchMap(({bucketId, password, privateSession}) => this.api.login(bucketId, password, privateSession).pipe(
      switchMap((auth) => [
        loginActions.loginSuccess({auth}),
        authActions.addLogin({auth})
      ]),
      catchError(async failure => loginActions.loginFailure({failure}))
    ))
  ));

  logout$ = createEffect(() => this.actions$.pipe(
    ofType(loginActions.logout),
    map(({auth}) => authActions.logout({auth}))
  ))

  public constructor(private actions$: Actions, private api: ApiService) {
  }
}
