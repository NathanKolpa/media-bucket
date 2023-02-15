import {NgModule} from '@angular/core';
import {StoreModule} from "@ngrx/store";
import {name, reducer} from './search.reducer';
import {EffectsModule} from "@ngrx/effects";
import {SearchEffects} from "./search.effects";

@NgModule({
  declarations: [],
  imports: [
    StoreModule.forFeature(name, reducer),
    EffectsModule.forFeature([SearchEffects])
  ]
})
export class SearchStoreModule {
}
