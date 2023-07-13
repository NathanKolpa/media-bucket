import {createAction, props} from "@ngrx/store";
import {Auth, Chart, ChartSeries, ChartSeriesQuery, Failure, SelectedBucket} from "@core/models";

export const reset = createAction('[Stats] reset');

export const openCreateQueryModal = createAction('[Stats] open create query modal');
export const addSeriesQuery = createAction('[Stats] add series query', props<{ query: ChartSeriesQuery }>());

export const loadChart = createAction('[Stats] load chart', props<{ bucket: SelectedBucket }>());
export const loadChartSuccess = createAction('[Stats] load chart success', props<{ chart: Chart }>());
export const loadChartFailure = createAction('[Stats] load chart failure', props<{ failure: Failure }>());
