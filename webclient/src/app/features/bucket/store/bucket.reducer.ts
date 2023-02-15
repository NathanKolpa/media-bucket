import {createFeature, createReducer, on} from "@ngrx/store";
import {Bucket, LoadingState} from "@core/models";
import * as bucketActions from './bucket.actions';

interface State {
  bucketLoadingState: LoadingState
  bucket: Bucket | null
}


const initialState: State = {
  bucketLoadingState: LoadingState.initial(),
  bucket: null
};

const feature = createFeature({
  name: 'bucket',
  reducer: createReducer(
    initialState,
    on(bucketActions.loadBucket, (state) => ({
      ...state,
      bucketLoadingState: state.bucketLoadingState.loading()
    })),
    on(bucketActions.loadBucketSuccess, (state, {bucket}) => ({
      ...state,
      bucketLoadingState: state.bucketLoadingState.success(),
      bucket
    })),
    on(bucketActions.loadBucketFailure, (state, {failure}) => ({
      ...state,
      bucketLoadingState: state.bucketLoadingState.fail(failure)
    })),
  )
});

export const {
  name,
  reducer,
  selectBucket,
  selectBucketLoadingState,
  selectBucketState
} = feature;

