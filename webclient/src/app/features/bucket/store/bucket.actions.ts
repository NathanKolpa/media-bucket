import { createAction, props } from "@ngrx/store";
import { Auth, Bucket, BucketDetails, Failure } from "@core/models";

export const logout = createAction('[Bucket] logout', props<{ auth: Auth }>())

export const loadBucket = createAction('[Bucket] load bucket', props<{ id: number }>());
export const loadBucketSuccess = createAction('[Bucket] load bucket success', props<{ bucket: Bucket }>());
export const loadBucketFailure = createAction('[Bucket] load bucket failure', props<{ failure: Failure }>());
export const reset = createAction('[Bucket] reset');

export const showGeneralInfo = createAction('[Bucket] show general info', props<{ auth: Auth }>());
export const loadBucketDetails = createAction('[Bucket] load bucket details', props<{ auth: Auth }>());
export const loadBucketDetailsSuccess = createAction('[Bucket] load bucket details success', props<{ details: BucketDetails }>());
export const loadBucketDetailsFailure = createAction('[Bucket] load bucket details failure', props<{ failure: Failure }>());

export const relogin = createAction('[Bucket] re-login', props<{ bucket: Bucket, oldAuth: Auth | null, password: string | null }>());
export const reloginSuccess = createAction('[Bucket] re-login success', props<{ bucket: Bucket, auth: Auth }>());
export const reloginFailure = createAction('[Bucket] re-login failure', props<{ failure: Failure }>());
