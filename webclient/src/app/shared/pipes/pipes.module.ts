import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {DurationPipe} from "@shared/pipes/duration.pipe";
import { BytesPipe } from './bytes.pipe';



@NgModule({
  declarations: [
    DurationPipe,
    BytesPipe
  ],
  exports: [
    DurationPipe,
    BytesPipe
  ],
  imports: [
    CommonModule
  ]
})
export class PipesModule { }
