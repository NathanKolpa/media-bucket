import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { ActionReducer, MetaReducer, StoreModule } from '@ngrx/store';
import { EffectsModule } from '@ngrx/effects';
import { HttpClientModule, } from "@angular/common/http";
import { CoreModule } from "@core/core.module";
import { environment } from "@src/environments/environment";

let showEventsOverride = false;

(window as any).showEvents = function() {
  showEventsOverride = true;
  console.log('Enabled debug events!')
}

function debug(reducer: ActionReducer<any>): ActionReducer<any> {
  return function(state, action) {
    const result = reducer(state, action);

    if (!environment.production || showEventsOverride) {
      console.groupCollapsed(action.type);
      console.log('prev state', state);
      console.log('action', action);
      console.log('next state', result);
      console.groupEnd();
    }

    return result;
  };
}

const metaReducers: MetaReducer[] = [debug];


@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    HttpClientModule,

    CoreModule,

    StoreModule.forRoot({}, { metaReducers }),
    EffectsModule.forRoot([]),
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
}
