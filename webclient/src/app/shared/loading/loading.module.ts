import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LoadingSpinnerComponent } from './components/loading-spinner/loading-spinner.component';
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import { LoadingErrorComponent } from './components/loading-error/loading-error.component';
import { LoadingLayoutComponent } from './components/loading-layout/loading-layout.component';
import {MatButtonModule} from "@angular/material/button";
import { LoadableContentComponent } from './components/loadable-content/loadable-content.component';



@NgModule({
  declarations: [
    LoadingSpinnerComponent,
    LoadingErrorComponent,
    LoadingLayoutComponent,
    LoadableContentComponent
  ],
  exports: [
    LoadingSpinnerComponent,
    LoadingErrorComponent,
    LoadableContentComponent
  ],
  imports: [
    CommonModule,
    MatProgressSpinnerModule,
    MatButtonModule
  ]
})
export class LoadingModule { }
