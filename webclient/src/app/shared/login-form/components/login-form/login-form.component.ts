import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output, } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { Bucket, LoadingState, } from "@core/models";

export interface Login {
  bucketId: number;
  password: string | null;
}

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-login-form',
  templateUrl: './login-form.component.html',
  styleUrls: ['./login-form.component.scss']
})
export class LoginFormComponent {
  form = new FormGroup({
    password: new FormControl('')
  })

  private _bucket: Bucket | null = null;

  @Input()
  public loadingState: LoadingState | null = null;

  public get bucket(): Bucket | null {
    return this._bucket;
  }

  @Input()
  public set bucket(value: Bucket | null) {
    this._bucket = value;

    if (!value) {
      return;
    }

    if (value.passwordProtected) {
      this.form.controls.password.setValidators([Validators.required]);
    } else {
      this.form.controls.password.clearValidators();
    }
  }

  @Output()
  public login = new EventEmitter<Login>();

  public hidePassword = true;

  public togglePasswordVisibility() {
    this.hidePassword = !this.hidePassword;
  }


  public loginClick() {
    if (this.form.valid && this.bucket) {
      let password = this.form.controls.password.value;

      this.login.emit({
        bucketId: this.bucket.id,
        password: password == '' ? null : password
      })
    }
  }
}
