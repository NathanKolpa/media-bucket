import {createSelector} from "@ngrx/store";
import {authSelectors, selectAuthState} from "@core/store/auth/auth.reducer";
import {bucketSelectors, selectLoginState} from "@features/login/store/login.reducer";
import {AuthenticatedBucket} from "@core/models";

export const selectBuckets = createSelector(
  selectAuthState,
  selectLoginState,

  (auth, login) => {
    let buckets = bucketSelectors.selectAll(login.buckets);

    return buckets.map(bucket => {
      let bucketAuth = authSelectors.selectEntities(auth.auth)[bucket.id] ?? null;

      return {
        bucket,
        auth: bucketAuth
      } as AuthenticatedBucket;
    });
  }
)

export {
  selectBucketsLoadingState,
  selectLoginLoadingState,
} from './login.reducer'
