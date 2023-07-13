import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {RouterModule, Routes} from "@angular/router";
import { StatsPageComponent } from './pages/stats-page/stats-page.component';
import { NgChartsModule } from 'ng2-charts';
import {StatsStoreModule} from "./store";
import { ChartComponent } from './components/chart/chart.component';
import {MatToolbarModule} from "@angular/material/toolbar";
import { ChartQueriesComponent } from './components/chart-queries/chart-queries.component';
import {MatInputModule} from "@angular/material/input";
import {MatChipsModule} from "@angular/material/chips";
import {MatButtonModule} from "@angular/material/button";
import {MatIconModule} from "@angular/material/icon";
import { QueryEditModalComponent } from './containers/query-edit-modal/query-edit-modal.component';
import {MatDialogModule} from "@angular/material/dialog";
import { QueryAddModalComponent } from './containers/query-add-modal/query-add-modal.component';
import { QueryFormComponent } from './components/query-form/query-form.component';
import {ReactiveFormsModule} from "@angular/forms";
import {LoadingModule} from "@shared/loading/loading.module";

const routes: Routes = [
  {
    path: '',
    component: StatsPageComponent,
  }
]

@NgModule({
  declarations: [
    StatsPageComponent,
    ChartComponent,
    ChartQueriesComponent,
    QueryEditModalComponent,
    QueryAddModalComponent,
    QueryFormComponent,
  ],
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
    StatsStoreModule,

    NgChartsModule,
    MatToolbarModule,
    MatInputModule,
    MatChipsModule,
    MatButtonModule,
    MatIconModule,
    MatDialogModule,
    ReactiveFormsModule,
    LoadingModule
  ]
})
export class StatsModule { }
