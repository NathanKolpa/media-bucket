import {createAction, props} from "@ngrx/store";
import {Chart, ChartDiscriminator, ChartSeriesQuery, Failure, SelectedBucket} from "@core/models";

export const reset = createAction('[Stats] reset');

export const openCreateQueryModal = createAction('[Stats] open create query modal');
export const addSeriesQuery = createAction('[Stats] add series query', props<{ query: ChartSeriesQuery }>());
export const updateQueryTitle = createAction('[Stats] update query title', props<{ title: string | null }>());
export const updateQueryDiscriminator = createAction('[Stats] update query discriminator', props<{ discriminator: ChartDiscriminator }>());
export const removeSeriesQuery = createAction('[Stats] remove series query', props<{ index: number }>());

export const loadChart = createAction('[Stats] load chart', props<{ bucket: SelectedBucket }>());
export const loadChartSuccess = createAction('[Stats] load chart success', props<{ chart: Chart }>());
export const loadChartFailure = createAction('[Stats] load chart failure', props<{ failure: Failure }>());
