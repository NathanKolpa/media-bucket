import {createAction, props} from "@ngrx/store";
import {Auth, Bucket, Failure} from "@core/models";

export const getAllBuckets = createAction('[Login] get all buckets');
export const getAllBucketsSuccess = createAction('[Login] get all buckets success', props<{ buckets: Bucket[] }>());
export const getAllBucketsFailure = createAction('[Login] get all buckets failure', props<{ failure: Failure }>());

export const login = createAction('[Login] login', props<{ bucketId: number, password: string | null, privateSession: boolean }>());
export const loginSuccess = createAction('[Login] login success', props<{ auth: Auth }>());
export const loginFailure = createAction('[Login] login failure', props<{ failure: Failure }>());

export const logout = createAction('[Login] logout', props<{ auth: Auth }>());
