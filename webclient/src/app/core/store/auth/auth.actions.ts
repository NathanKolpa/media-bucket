import { createAction, props } from "@ngrx/store";
import { Auth, Failure } from "@core/models";

export const initialize = createAction('[Auth] initialize');
export const initializeSuccess = createAction('[Auth] initialize success', props<{ auth: Auth[] }>());

export const addLogin = createAction('[Auth] add login', props<{ auth: Auth }>());

export const logout = createAction('[Auth] logout', props<{ auth: Auth }>());
export const logoutSuccess = createAction('[Auth] logout success');
export const logoutFailure = createAction('[Auth] logout failure', props<{ failure: Failure }>());

export const failedAuth = createAction('[Auth] failed auth', props<{ failure: Failure, url: string | null }>())
