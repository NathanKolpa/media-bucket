import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Auth, AuthenticatedBucket, LoadingState} from "@core/models";
import {FormControl, FormGroup, Validators} from "@angular/forms";

export interface Login {
  bucketId: number;
  password: string | null;
}

export interface SelectBucket {
  bucketId: number;
}

export interface Logout {
  auth: Auth
}

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-login-form',
  templateUrl: './login-form.component.html',
  styleUrls: ['./login-form.component.scss']
})
export class LoginFormComponent {

  @Output()
  public login = new EventEmitter<Login>();

  @Output()
  public logout = new EventEmitter<Logout>();

  @Output()
  public selectBucket = new EventEmitter<SelectBucket>();

  @Input()
  public loginLoading: LoadingState | null = null;
  public selectedId = 0;
  public hidePassword = true;
  form = new FormGroup({
    password: new FormControl('')
  })
  private firstLoad = true;

  private _buckets: AuthenticatedBucket[] | null = null;

  public get buckets(): AuthenticatedBucket[] | null {
    return this._buckets;
  }

  @Input()
  public set buckets(value: AuthenticatedBucket[] | null) {
    this._buckets = value;

    if (this.firstLoad && value !== null && value.length > 0) {
      this.setSelectedId(value[0].bucket.id);
      this.firstLoad = false;
    }
  }

  public get selectedBucket(): AuthenticatedBucket | null {
    return this.buckets?.find(x => x.bucket.id == this.selectedId) ?? null;
  }

  public setSelectedId(id: number) {
    this.selectedId = id;
    this.hidePassword = true;
    this.form.reset();

    let bucket = this.selectedBucket;

    if (bucket) {
      if (bucket.bucket.passwordProtected) {
        this.form.controls.password.enable();
        this.form.controls.password.setValidators(Validators.required);
      } else {
        this.form.controls.password.disable();
        this.form.controls.password.clearValidators();
      }
    }
  }

  public togglePasswordVisibility() {
    this.hidePassword = !this.hidePassword;
  }

  public logoutClick() {
    let bucket = this.selectedBucket;

    if (bucket?.auth) {
      this.logout.emit({
        auth: bucket.auth
      })
    }

  }

  public selectClick() {
    this.selectBucket.emit({
      bucketId: this.selectedId
    })
  }

  public loginClick() {
    if (this.form.valid) {
      let password = this.form.controls.password.value;

      this.login.emit({
        bucketId: this.selectedId,
        password: password == '' ? null : password
      })
    }
  }
}
