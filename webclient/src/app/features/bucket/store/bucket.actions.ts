import {createAction, props} from "@ngrx/store";
import {Auth, Bucket, Failure} from "@core/models";

export const logout = createAction('[Bucket] logout', props<{ auth: Auth }>())

export const loadBucket = createAction('[Bucket] load bucket', props<{ id: number }>());
export const loadBucketSuccess = createAction('[Bucket] load bucket success', props<{ bucket: Bucket }>());
export const loadBucketFailure = createAction('[Bucket] load bucket failure', props<{ failure: Failure }>());
