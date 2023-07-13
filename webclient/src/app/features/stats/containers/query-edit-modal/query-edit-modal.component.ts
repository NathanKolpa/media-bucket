import {Component} from '@angular/core';
import {Store} from "@ngrx/store";

@Component({
  selector: 'app-query-edit-modal',
  templateUrl: './query-edit-modal.component.html',
  styleUrls: ['./query-edit-modal.component.scss']
})
export class QueryEditModalComponent {
  constructor(private store: Store) {
  }
}
