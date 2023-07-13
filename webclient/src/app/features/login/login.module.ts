import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {LoginPageComponent} from './pages/login-page/login-page.component';
import {RouterModule, Routes} from "@angular/router";
import {LoginTileComponent} from './components/login-tile/login-tile.component';
import {LoginLayoutComponent} from './components/login-layout/login-layout.component';
import {MatCardModule} from "@angular/material/card";
import {LoginFormComponent} from './components/login-form/login-form.component';
import {MatInputModule} from "@angular/material/input";
import {ReactiveFormsModule} from "@angular/forms";
import {MatIconModule} from "@angular/material/icon";
import {MatButtonModule} from "@angular/material/button";
import {BucketSelectComponent} from './components/bucket-select/bucket-select.component';
import {MatListModule} from "@angular/material/list";
import {LoginStoreModule} from "./store";
import {LoadingModule} from "@shared/loading/loading.module";

const routes: Routes = [
  {
    path: '',
    pathMatch: 'full',
    component: LoginPageComponent
  }
]

@NgModule({
  declarations: [
    LoginPageComponent,
    LoginTileComponent,
    LoginLayoutComponent,
    LoginFormComponent,
    BucketSelectComponent,
  ],
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
    LoginStoreModule,

    ReactiveFormsModule,
    MatCardModule,
    MatInputModule,
    MatListModule,
    MatButtonModule,
    MatIconModule,
    LoadingModule,
  ]
})
export class LoginModule {
}
