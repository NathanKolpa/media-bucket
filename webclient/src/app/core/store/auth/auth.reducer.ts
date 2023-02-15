import {createFeature, createReducer, on} from "@ngrx/store";
import {createEntityAdapter, EntityState} from "@ngrx/entity";
import {Auth} from "@core/models";
import {addLogin} from "@core/store/auth/auth.actions";

import * as authActions from './auth.actions';


interface State {
  auth: EntityState<Auth>
}

const authAdapter = createEntityAdapter<Auth>({
  selectId: x => x.bucketId
});

const initialState: State = {
  auth: authAdapter.getInitialState()
};

const feature = createFeature({
  name: 'auth',
  reducer: createReducer(
    initialState,
    on(authActions.addLogin, (state, {auth}) => ({
      ...state,
      auth: authAdapter.setOne(auth, state.auth)
    })),

    on(authActions.logout, (state, {auth}) => ({
      ...state,
      auth: authAdapter.removeOne(auth.bucketId, state.auth)
    })),

    on(authActions.initializeSuccess, (state, {auth}) => ({
      ...state,
      auth: authAdapter.setAll(auth, state.auth)
    }))
  )
});

export const {
  name,
  reducer,
  selectAuthState
} = feature;

export const authSelectors = authAdapter.getSelectors();

