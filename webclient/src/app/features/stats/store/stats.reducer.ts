import {createFeature, createReducer, on} from "@ngrx/store";
import {Chart, ChartQuery, LoadingState} from "@core/models";
import * as statsActions from "./stats.actions";

interface State {
  query: ChartQuery,
  chart: Chart | null,
  loadingState: LoadingState
}


const initialState: State = {
  query: ChartQuery.initial(),
  loadingState: LoadingState.initial(),
  chart: null
}

const feature = createFeature({
  name: 'stats',
  reducer: createReducer(
    initialState,
    on(statsActions.addSeriesQuery, (state, {query}) => ({
      ...state,
      query: state.query.addSeries(query)
    })),
    on(statsActions.loadChart, (state) => ({
      ...state,
      loadingState: state.loadingState.loading()
    })),
    on(statsActions.loadChartFailure, (state, {failure}) => ({
      ...state,
      loadingState: state.loadingState.fail(failure)
    })),
    on(statsActions.loadChartSuccess, (state, {chart}) => ({
      ...state,
      loadingState: state.loadingState.success(),
      chart
    })),
    on(statsActions.updateQueryTitle, (state, { title }) => ({
      ...state,
      query: state.query.setTitle(title)
    })),
    on(statsActions.updateQueryDiscriminator, (state, { discriminator }) => ({
      ...state,
      query: state.query.setDiscriminator(discriminator)
    })),
    on(statsActions.removeSeriesQuery, (state, { index }) => ({
      ...state,
      query: state.query.removeSeries(index)
    }))
  )
});

export const {
  name,
  reducer,
  selectStatsState,
  selectLoadingState,
  selectChart,
  selectQuery
} = feature;
