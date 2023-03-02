import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {DurationPipe} from "@shared/pipes/duration.pipe";
import { BytesPipe } from './bytes.pipe';
import { AgoPipe } from './ago.pipe';



@NgModule({
  declarations: [
    DurationPipe,
    BytesPipe,
    AgoPipe
  ],
  exports: [
    DurationPipe,
    BytesPipe,
    AgoPipe
  ],
  imports: [
    CommonModule
  ]
})
export class PipesModule { }
