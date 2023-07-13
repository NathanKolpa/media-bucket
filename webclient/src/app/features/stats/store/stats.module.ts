import {NgModule} from '@angular/core';
import {StoreModule} from "@ngrx/store";
import {name, reducer} from './stats.reducer';
import {EffectsModule} from "@ngrx/effects";
import {StatsEffects} from "./stats.effects";

@NgModule({
  declarations: [],
  imports: [
    StoreModule.forFeature(name, reducer),
    EffectsModule.forFeature([StatsEffects])
  ]
})
export class StatsStoreModule {
}
