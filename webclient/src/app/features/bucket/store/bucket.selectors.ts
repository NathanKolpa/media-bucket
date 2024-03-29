import { createSelector } from "@ngrx/store";
import { authSelectors, selectAuthState } from "@core/store/auth/auth.reducer";
import { SelectedBucket } from "@core/models";
import * as reducer from "./bucket.reducer";

export const selectShallowBucket = reducer.selectBucket;

export const selectBucket = createSelector(
  selectAuthState,
  reducer.selectBucket,

  (auth, bucket) => {

    if (!bucket) {
      return null;
    }

    let bucketAuth = authSelectors.selectEntities(auth.auth)[bucket.id] ?? null;

    if (bucketAuth == null) {
      return null;
    }

    return {
      bucket,
      auth: bucketAuth
    } as SelectedBucket;
  }
);
export const isAuthenticated = createSelector(
  selectAuthState,
  reducer.selectBucket,

  (auth, bucket) => {
    if (!bucket) {
      return false;
    }

    let bucketAuth = authSelectors.selectEntities(auth.auth)[bucket.id] ?? null;

    return bucketAuth !== null;
  }
);

export {
  selectBucketLoadingState,
  selectDetails,
  selectDetailsLoadingState,
  selectReloginLoadingState
} from './bucket.reducer'
