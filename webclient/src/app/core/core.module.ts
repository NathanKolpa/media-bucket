import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {ApiService, AppTitleService, AuthCacheService, ConfirmGuard} from "./services";
import {AuthStoreModule} from "./store/auth";


@NgModule({
  imports: [
    CommonModule,
    AuthStoreModule
  ],
  providers: [
    ApiService,
    AuthCacheService,
    AppTitleService,
    ConfirmGuard,
  ]
})
export class CoreModule {
}
