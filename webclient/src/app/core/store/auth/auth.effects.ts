import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {ApiService, AuthCacheService} from "@core/services";
import * as authActions from './auth.actions';
import {catchError, map, switchMap, tap} from "rxjs";

@Injectable()
export class AuthEffects {

  addLogin$ = createEffect(() => this.actions$.pipe(
    ofType(authActions.addLogin),
    tap(({auth}) => {
      this.authCache.store(auth);
    })
  ), {dispatch: false});

  logout$ = createEffect(() => this.actions$.pipe(
    ofType(authActions.logout),
    tap(({auth}) => {
      this.authCache.remove(auth);
    }),
    map(() => authActions.logoutSuccess())
  ));

  initialize$ = createEffect(() => this.actions$.pipe(
    ofType(authActions.initialize),
    map(() => authActions.initializeSuccess({auth: this.authCache.load()}))
  ));

  public constructor(private actions$: Actions, private api: ApiService, private authCache: AuthCacheService) {
  }
}
