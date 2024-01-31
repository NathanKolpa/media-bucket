import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LoginFormComponent } from "./components/login-form/login-form.component";
import { MatInputModule } from "@angular/material/input";
import { MatIconModule } from "@angular/material/icon";
import { ReactiveFormsModule } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatButtonModule } from '@angular/material/button';



@NgModule({
  declarations: [
    LoginFormComponent
  ],
  imports: [
    CommonModule,

    ReactiveFormsModule,
    MatFormFieldModule,
    MatIconModule,
    MatInputModule,
    MatButtonModule

  ],
  exports: [
    LoginFormComponent
  ]
})
export class LoginFormModule { }
