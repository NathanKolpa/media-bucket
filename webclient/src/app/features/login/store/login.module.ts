import {NgModule} from '@angular/core';
import {StoreModule} from "@ngrx/store";
import {name, reducer} from './login.reducer';
import {EffectsModule} from "@ngrx/effects";
import {LoginEffects} from "./login.effects";

@NgModule({
  declarations: [],
  imports: [
    StoreModule.forFeature(name, reducer),
    EffectsModule.forFeature([LoginEffects])
  ]
})
export class LoginStoreModule {
}
