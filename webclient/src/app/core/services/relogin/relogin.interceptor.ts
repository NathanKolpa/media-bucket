import { Injectable } from '@angular/core';
import {
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpInterceptor,
  HttpErrorResponse
} from '@angular/common/http';
import { catchError, Observable, throwError } from 'rxjs';
import { Store } from '@ngrx/store';
import { authActions } from '@core/store/auth';

@Injectable()
export class ReloginInterceptor implements HttpInterceptor {

  constructor(private store: Store) { }

  intercept(request: HttpRequest<unknown>, next: HttpHandler): Observable<HttpEvent<unknown>> {
    return next.handle(request).pipe(catchError((err: any, _caught) => {
      if (err instanceof HttpErrorResponse && err.url !== null && !err.url.endsWith('/auth')) {
        this.store.dispatch(authActions.failedAuth({ failure: err.error, url: err.url }))
      }
      return throwError(() => err)
    }));
  }
}
