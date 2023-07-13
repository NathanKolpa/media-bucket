import { Component } from '@angular/core';
import { fromStats } from '@features/stats/store';
import {Store} from "@ngrx/store";
import {map} from "rxjs";

@Component({
  selector: 'app-query-edit-modal',
  templateUrl: './query-edit-modal.component.html',
  styleUrls: ['./query-edit-modal.component.scss']
})
export class QueryEditModalComponent {
  constructor(private store: Store) {
  }
}
