import {createFeature, createReducer, on} from "@ngrx/store";
import {Bucket, BucketDetails, LoadingState} from "@core/models";
import * as bucketActions from './bucket.actions';

interface State {
  bucketLoadingState: LoadingState
  bucket: Bucket | null,
  details: BucketDetails | null
  detailsLoadingState: LoadingState
}


const initialState: State = {
  bucketLoadingState: LoadingState.initial(),
  bucket: null,
  details: null,
  detailsLoadingState: LoadingState.initial()
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
    on(bucketActions.loadBucketDetails, (state) => ({
      ...state,
      detailsLoadingState: state.detailsLoadingState.loading()
    })),
    on(bucketActions.loadBucketDetailsSuccess, (state, {details}) => ({
      ...state,
      detailsLoadingState: state.detailsLoadingState.success(),
      details
    })),
    on(bucketActions.loadBucketDetailsFailure, (state, {failure}) => ({
      ...state,
      detailsLoadingState: state.detailsLoadingState.fail(failure)
    })),
  )
});

export const {
  name,
  reducer,
  selectBucket,
  selectBucketLoadingState,
  selectDetails,
  selectDetailsLoadingState,
  selectBucketState
} = feature;

