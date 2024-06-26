import { ChangeDetectionStrategy, Component, OnDestroy } from '@angular/core';
import { AppTitleService } from "@core/services";
import { bucketActions, fromBucket } from '@features/bucket/store';
import { Store } from "@ngrx/store";
import { filter, Subscription } from "rxjs";
import { ActivatedRoute, Router } from "@angular/router";
import { Auth, Bucket } from "@core/models";
import { Actions, ofType } from "@ngrx/effects";
import { Login } from '@shared/login-form/components/login-form/login-form.component';

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-bucket-page',
  templateUrl: './bucket-page.component.html',
  styleUrls: ['./bucket-page.component.scss']
})
export class BucketPageComponent implements OnDestroy {

  selectedBucket$ = this.store.select(fromBucket.selectBucket);
  bucket$ = this.store.select(fromBucket.selectShallowBucket);
  isAuthenticated$ = this.store.select(fromBucket.isAuthenticated);
  bucketLoadingState$ = this.store.select(fromBucket.selectBucketLoadingState);
  loginLoadingState$ = this.store.select(fromBucket.selectReloginLoadingState);

  private titleIndex: number | null = null;
  private bucketTitleSubscription: Subscription;
  private paramsSubscription: Subscription;
  private logoutNavigateSubscription: Subscription;

  constructor(private title: AppTitleService,
    private store: Store,
    private route: ActivatedRoute,
    private actions: Actions,
    private router: Router) {
    this.bucketTitleSubscription = this.selectedBucket$.pipe(filter(x => x !== null)).subscribe(bucket => {
      if (!bucket) {
        return;
      }

      if (this.titleIndex !== null) {
        this.title.set(this.titleIndex, bucket.bucket.name);
      } else {
        this.titleIndex = this.title.push(bucket.bucket.name);
      }
    });

    this.paramsSubscription = this.route.params.subscribe(params => this.load(params));

    this.logoutNavigateSubscription = this.actions.pipe(ofType(bucketActions.logout)).subscribe(() => {
      let _ = this.router.navigate(['/']);
    })
  }

  reload() {
    this.load(this.route.snapshot.params);
  }

  load(params: any) {
    let id = params['bucketId'];
    if (!!id && !isNaN(+id)) {
      this.store.dispatch(bucketActions.loadBucket({ id: +id }));
    }
  }

  ngOnDestroy(): void {
    if (this.titleIndex !== null) {
      this.title.pop();
    }

    this.bucketTitleSubscription.unsubscribe();
    this.paramsSubscription.unsubscribe();
    this.logoutNavigateSubscription.unsubscribe();

    this.store.dispatch(bucketActions.reset());
  }

  logout(auth: Auth) {
    this.store.dispatch(bucketActions.logout({ auth }));
  }

  showGeneralInfo(auth: Auth) {
    this.store.dispatch(bucketActions.showGeneralInfo({ auth }));
  }

  login(bucket: Bucket, req: Login) {
    this.store.dispatch(bucketActions.relogin({ bucket, oldAuth: null, password: req.password }));
  }
}
