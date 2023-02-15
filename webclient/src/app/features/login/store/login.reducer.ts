import {createFeature, createReducer, on} from "@ngrx/store";
import {createEntityAdapter, EntityState} from "@ngrx/entity";
import {Bucket, LoadingState} from "@core/models";
import * as loginActions from './login.actions';

interface State {
  bucketsLoadingState: LoadingState
  loginLoadingState: LoadingState

  buckets: EntityState<Bucket>;
}

const bucketAdapter = createEntityAdapter<Bucket>();

const initialState: State = {
  bucketsLoadingState: LoadingState.initial(),
  loginLoadingState: LoadingState.initial(),
  buckets: bucketAdapter.getInitialState()
};

const feature = createFeature({
  name: 'login',
  reducer: createReducer(
    initialState,

    // get all buckets
    on(loginActions.getAllBuckets, (state) => ({
      ...state,
      bucketsLoadingState: state.bucketsLoadingState.loading()
    })),
    on(loginActions.getAllBucketsSuccess, (state, {buckets}) => ({
      ...state,
      buckets: bucketAdapter.setAll(buckets, state.buckets),
      bucketsLoadingState: state.bucketsLoadingState.success()
    })),
    on(loginActions.getAllBucketsFailure, (state, {failure}) => ({
      ...state,
      bucketsLoadingState: state.bucketsLoadingState.fail(failure)
    })),

    // login
    on(loginActions.login, (state) => ({
      ...state,
      loginLoadingState: state.loginLoadingState.loading()
    })),
    on(loginActions.loginSuccess, (state) => ({
      ...state,
      loginLoadingState: state.loginLoadingState.success()
    })),
    on(loginActions.loginFailure, (state, {failure}) => ({
      ...state,
      loginLoadingState: state.loginLoadingState.fail(failure)
    })),
  )
});

export const {
  name,
  reducer,
  selectLoginState,
  selectBucketsLoadingState,
  selectLoginLoadingState
} = feature;

export const bucketSelectors = bucketAdapter.getSelectors();
