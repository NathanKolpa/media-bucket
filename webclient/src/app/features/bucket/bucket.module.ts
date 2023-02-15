import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {RouterModule, Routes} from "@angular/router";
import {BucketPageComponent} from './pages/bucket-page/bucket-page.component';
import {BucketStoreModule} from "./store";
import {LoadingModule} from "@shared/loading/loading.module";
import {HeaderComponent} from './components/header/header.component';
import {MatToolbarModule} from "@angular/material/toolbar";
import {MatButtonModule} from "@angular/material/button";
import {MatIconModule} from "@angular/material/icon";
import {MatMenuModule} from "@angular/material/menu";
import {NotAuthenticatedComponent} from './components/not-authenticated/not-authenticated.component';

const routes: Routes = [
  {
    path: '',
    component: BucketPageComponent,
    children: [
      {
        path: '',
        pathMatch: 'full',
        loadChildren: () => import("@features/search/search.module").then(m => m.SearchModule)
      }
    ]
  }
];

@NgModule({
  declarations: [
    BucketPageComponent,
    HeaderComponent,
    NotAuthenticatedComponent,
  ],
  imports: [
    CommonModule,
    RouterModule.forChild(routes),

    BucketStoreModule,
    LoadingModule,
    MatToolbarModule,
    MatButtonModule,
    MatMenuModule,
    MatIconModule,
  ]
})
export class BucketModule {
}
