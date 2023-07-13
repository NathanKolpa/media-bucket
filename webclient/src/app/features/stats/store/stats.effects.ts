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
    withLatestFrom(this.store.select(fromStats.selectQuery)),
    switchMap(([action, query]) => {
      return this.api.loadChart(action.bucket.auth, query).pipe(
        map(chart => statsActions.loadChartSuccess({chart})),
        catchError(async failure => statsActions.loadChartFailure({failure}))
      )
    })
  ))

  constructor(private actions$: Actions, private store: Store, private api: ApiService) {
  }
}
