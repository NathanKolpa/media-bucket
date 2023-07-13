import {createFeature, createReducer, on} from "@ngrx/store";
import {createEntityAdapter, EntityState} from "@ngrx/entity";
import {Chart, ChartsQuery, LoadingState} from "@core/models";
import * as statsActions from "./stats.actions";

interface State {
  title: string | null
  queries: EntityState<ChartsQuery>,
  charts: EntityState<Chart>,
  loadingState: LoadingState
}

const queryAdapter = createEntityAdapter<ChartsQuery>({
  selectId: (x) => x.name
});

const chartAdapter = createEntityAdapter<Chart>({
  selectId: (x) => x.name
});

const initialState: State = {
  title: null,
  queries: queryAdapter.getInitialState(),
  loadingState: LoadingState.initial(),
  charts: chartAdapter.getInitialState()
}

const feature = createFeature({
  name: 'stats',
  reducer: createReducer(
    initialState,
    on(statsActions.addQuery, (state, {query}) => ({
      ...state,
      queries: queryAdapter.addOne(query, state.queries)
    })),
    on(statsActions.loadChart, (state) => ({
      ...state,
      loadingState: state.loadingState.loading()
    })),
    on(statsActions.loadChartFailure, (state, { failure }) => ({
      ...state,
      loadingState: state.loadingState.fail(failure)
    })),
    on(statsActions.loadChartSuccess, (state, { charts }) => ({
      ...state,
      loadingState: state.loadingState.success(),
      charts: chartAdapter.setAll(charts, state.charts)
    })),
  )
});

export const {
  name,
  reducer,
  selectStatsState,
  selectTitle,
  selectLoadingState
} = feature;

export const querySelectors = queryAdapter.getSelectors();
export const chartSelectors = chartAdapter.getSelectors();
