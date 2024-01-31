import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ApiService, AppTitleService, AuthCacheService, ConfirmGuard, ReloginInterceptor } from "./services";
import { AuthStoreModule } from "./store/auth";
import { HTTP_INTERCEPTORS } from '@angular/common/http';


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
    {
      provide: HTTP_INTERCEPTORS, useClass: ReloginInterceptor, multi: true
    }
  ]
})
export class CoreModule {
}
