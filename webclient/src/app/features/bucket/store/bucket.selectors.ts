import {createSelector} from "@ngrx/store";
import {authSelectors, selectAuthState} from "@core/store/auth/auth.reducer";
import {AuthenticatedBucket, SelectedBucket} from "@core/models";
import * as reducer from "./bucket.reducer";
import {bucketSelectors, selectLoginState} from "@features/login/store/login.reducer";

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
  selectBucketLoadingState
} from './bucket.reducer'
