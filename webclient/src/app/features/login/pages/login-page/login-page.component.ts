import { ChangeDetectionStrategy, Component, OnDestroy, OnInit } from '@angular/core';
import { fromLogin, loginActions } from '@features/login/store';
import { Store } from "@ngrx/store";
import { Logout, SelectBucket } from "@features/login/components/login/login.component";
import { Router } from "@angular/router";
import { Actions, ofType } from "@ngrx/effects";
import { Subscription } from "rxjs";
import { Login } from '@shared/login-form/components/login-form/login-form.component';

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-login-page',
  templateUrl: './login-page.component.html',
  styleUrls: ['./login-page.component.scss']
})
export class LoginPageComponent implements OnInit, OnDestroy {

  navigateOnLoginSubscription: Subscription;

  buckets$ = this.store.select(fromLogin.selectBuckets);
  bucketsLoadingState$ = this.store.select(fromLogin.selectBucketsLoadingState);
  loginLoadingState$ = this.store.select(fromLogin.selectLoginLoadingState);

  constructor(private store: Store, private actions: Actions, private router: Router) {
    this.navigateOnLoginSubscription = this.actions.pipe(ofType(loginActions.loginSuccess)).subscribe(({ auth }) => {
      this.viewBucket(auth.bucketId);
    })
  }

  ngOnDestroy(): void {
    this.navigateOnLoginSubscription.unsubscribe();
  }

  ngOnInit(): void {
    this.load();
  }

  load() {
    this.store.dispatch(loginActions.getAllBuckets());
  }

  login(req: Login) {
    this.store.dispatch(loginActions.login({ bucketId: req.bucketId, password: req.password, privateSession: false }));
  }

  logout(req: Logout) {
    this.store.dispatch(loginActions.logout({ auth: req.auth }))
  }

  select(req: SelectBucket) {
    this.viewBucket(req.bucketId);
  }

  viewBucket(id: number) {
    let _ = this.router.navigate(['buckets', id])
  }
}
