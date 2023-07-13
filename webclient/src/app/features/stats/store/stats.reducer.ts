import {createFeature, createReducer} from "@ngrx/store";

interface State {
}

const initialState: State = {
}

const feature = createFeature({
  name: 'stats',
  reducer: createReducer(
    initialState,
  )
});

export const {
  name,
  reducer,
  selectStatsState,
} = feature;
