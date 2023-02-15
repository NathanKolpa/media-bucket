import {NgModule} from '@angular/core';
import {StoreModule} from "@ngrx/store";
import {name, reducer} from './auth.reducer';
import {EffectsModule} from "@ngrx/effects";
import {AuthEffects} from "./auth.effects";

@NgModule({
  declarations: [],
  imports: [
    StoreModule.forFeature(name, reducer),
    EffectsModule.forFeature([AuthEffects])
  ]
})
export class AuthStoreModule {
}
