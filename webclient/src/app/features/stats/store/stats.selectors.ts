import {createSelector} from "@ngrx/store";
import {selectStatsState, querySelectors} from "./stats.reducer";

export const selectQueries = createSelector(
  selectStatsState,
  (state) => {
    return querySelectors.selectAll(state.queries)
  }
)

export const selectHasChart = createSelector(
  selectStatsState,
  (state) => {
    return false;
  }
)

export {
  selectStatsState,
  selectTitle,
  selectLoadingState,
} from './stats.reducer'

