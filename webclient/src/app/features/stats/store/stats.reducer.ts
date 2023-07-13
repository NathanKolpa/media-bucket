import {createFeature, createReducer, on} from "@ngrx/store";
import {createEntityAdapter, EntityState} from "@ngrx/entity";
import {ChartsQuery, LoadingState} from "@core/models";
import * as statsActions from "./stats.actions";

interface State {
  title: string | null
  queries: EntityState<ChartsQuery>,
  loadingState: LoadingState
}

const queryAdapter = createEntityAdapter<ChartsQuery>({
  selectId: (x) => x.name
});


const initialState: State = {
  title: null,
  queries: queryAdapter.getInitialState(),
  loadingState: LoadingState.initial()
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
      loadingState: state.loadingState.success()
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
