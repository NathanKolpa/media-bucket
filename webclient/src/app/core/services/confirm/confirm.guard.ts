import { Injectable } from '@angular/core';
import {ActivatedRouteSnapshot, CanDeactivate, RouterStateSnapshot, UrlTree} from '@angular/router';
import { Observable } from 'rxjs';

export interface ConfirmComponent {
  showNavigationWarning(): boolean | string;
}


@Injectable({
  providedIn: 'root'
})
export class ConfirmGuard implements CanDeactivate<any> {
  canDeactivate(component: ConfirmComponent, currentRoute: ActivatedRouteSnapshot, currentState: RouterStateSnapshot, nextState?: RouterStateSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {
    if (component == null) {
      return true;
    }

    let showWarning = component.showNavigationWarning();

    let warningMessage = 'Are you sure you want to leave the page?';

    if(typeof showWarning == 'string') {
      warningMessage = showWarning;
    }

    if (showWarning || typeof showWarning == 'string') {
      return confirm(warningMessage);
    }

    return true;
  }

}
