import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import * as statsActions from "./stats.actions";
import * as fromStats from "./stats.selectors";
import {catchError, forkJoin, map, switchMap, withLatestFrom} from "rxjs";
import {Store} from "@ngrx/store";
import {ApiService} from "@core/services";

@Injectable()
export class StatsEffects {
  loadChart$ = createEffect(() => this.actions$.pipe(
    ofType(statsActions.loadChart),
    withLatestFrom(this.store.select(fromStats.selectQueries)),
    switchMap(([action, queries]) => {
      let apiCalls = queries.map(query => this.api.loadChart(action.bucket.auth, query));

      return forkJoin(apiCalls).pipe(
        map(charts => statsActions.loadChartSuccess({charts})),
        catchError(async failure => statsActions.loadChartFailure({failure}))
      )
    })
  ))

  constructor(private actions$: Actions, private store: Store, private api: ApiService) {
  }
}
