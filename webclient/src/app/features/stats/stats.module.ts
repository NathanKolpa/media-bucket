import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {RouterModule, Routes} from "@angular/router";
import { StatsPageComponent } from './pages/stats-page/stats-page.component';
import { NgChartsModule } from 'ng2-charts';
import {StatsStoreModule} from "./store";

const routes: Routes = [
  {
    path: '',
    component: StatsPageComponent,
  }
]

@NgModule({
  declarations: [
    StatsPageComponent
  ],
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
    StatsStoreModule,

    NgChartsModule
  ]
})
export class StatsModule { }
