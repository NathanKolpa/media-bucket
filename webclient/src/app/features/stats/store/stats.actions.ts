import {createAction, props} from "@ngrx/store";
import {Auth, Chart, ChartsQuery, Failure, SelectedBucket} from "@core/models";

export const reset = createAction('[Stats] reset');

export const updateTitle = createAction('[Stats] update title', props<{ title: string | null }>());

export const openCreateQueryModal = createAction('[Stats] open create query modal');

export const addQuery = createAction('[Stats] add query', props<{ query: ChartsQuery }>());

export const loadChart = createAction('[Stats] load chart', props<{ bucket: SelectedBucket }>());
export const loadChartSuccess = createAction('[Stats] load chart success', props<{ charts: Chart[] }>());
export const loadChartFailure = createAction('[Stats] load chart failure', props<{ failure: Failure }>());
