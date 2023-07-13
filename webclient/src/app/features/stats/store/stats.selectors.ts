import {createSelector} from "@ngrx/store";
import {selectStatsState, querySelectors, chartSelectors} from "./stats.reducer";

export const selectQueries = createSelector(
  selectStatsState,
  (state) => {
    return querySelectors.selectAll(state.queries)
  }
)

export const selectCharts = createSelector(
  selectStatsState,
  (state) => {
    return chartSelectors.selectAll(state.charts)
  }
)


export const selectHasChart = createSelector(
  selectStatsState,
  (state) => {
    return chartSelectors.selectTotal(state.charts) > 0;
  }
)

export {
  selectStatsState,
  selectTitle,
  selectLoadingState,
} from './stats.reducer'

