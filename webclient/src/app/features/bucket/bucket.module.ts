import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, Routes } from "@angular/router";
import { BucketPageComponent } from './pages/bucket-page/bucket-page.component';
import { BucketStoreModule } from "./store";
import { LoadingModule } from "@shared/loading/loading.module";
import { HeaderComponent } from './components/header/header.component';
import { MatToolbarModule } from "@angular/material/toolbar";
import { MatButtonModule } from "@angular/material/button";
import { MatIconModule } from "@angular/material/icon";
import { MatMenuModule } from "@angular/material/menu";
import { NotAuthenticatedComponent } from './components/not-authenticated/not-authenticated.component';
import { MatDividerModule } from "@angular/material/divider";
import { MatDialogModule } from "@angular/material/dialog";
import { BucketDetailsDialogComponent } from './containers/bucket-details-dialog/bucket-details-dialog.component';
import { PipesModule } from "@shared/pipes/pipes.module";
import { MatTooltipModule } from "@angular/material/tooltip";
import { ReloginModalComponent } from './containers/relogin-modal/relogin-modal.component';
import { LoginFormModule } from '@shared/login-form/login-form.module';

const routes: Routes = [
  {
    path: '',
    component: BucketPageComponent,
    children: [
      {
        path: '',
        pathMatch: 'full',
        loadChildren: () => import("@features/search/search.module").then(m => m.SearchModule)
      },
      {
        path: 'stats',
        pathMatch: 'full',
        loadChildren: () => import("@features/stats/stats.module").then(m => m.StatsModule)
      }
    ]
  }
];

@NgModule({
  declarations: [
    BucketPageComponent,
    HeaderComponent,
    NotAuthenticatedComponent,
    BucketDetailsDialogComponent,
    ReloginModalComponent,
  ],
  imports: [
    CommonModule,
    RouterModule.forChild(routes),

    LoginFormModule,

    BucketStoreModule,
    LoadingModule,
    MatToolbarModule,
    MatButtonModule,
    MatMenuModule,
    MatIconModule,
    MatDividerModule,
    MatDialogModule,
    PipesModule,
    MatTooltipModule
  ],
})
export class BucketModule {
}
