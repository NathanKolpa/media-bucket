import {createSelector} from "@ngrx/store";
import {selectStatsState} from "./stats.reducer";

export const selectSeries = createSelector(
  selectStatsState,
  (state) => {
    return state.query.series
  }
)

export const selectHasChart = createSelector(
  selectStatsState,
  (state) => {
    return state.chart !== null
  }
)

export {
  selectStatsState,
  selectLoadingState,
  selectChart,
  selectQuery
} from './stats.reducer'

