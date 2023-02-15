import {NgModule} from '@angular/core';
import {StoreModule} from "@ngrx/store";
import {name, reducer} from './bucket.reducer';
import {EffectsModule} from "@ngrx/effects";
import {BucketEffects} from "./bucket.effects";

@NgModule({
  declarations: [],
  imports: [
    StoreModule.forFeature(name, reducer),
    EffectsModule.forFeature([BucketEffects])
  ]
})
export class BucketStoreModule {
}
