import { Component } from '@angular/core';
import {MatDialogRef} from "@angular/material/dialog";
import {Store} from "@ngrx/store";
import {ChartsQuery} from "@core/models";
import { statsActions } from '@features/stats/store';

@Component({
  selector: 'app-query-add-modal',
  templateUrl: './query-add-modal.component.html',
  styleUrls: ['./query-add-modal.component.scss']
})
export class QueryAddModalComponent {
  constructor(private dialogRef: MatDialogRef<QueryAddModalComponent>, private store: Store) {
  }

  addQuery(query: ChartsQuery) {
    this.store.dispatch(statsActions.addQuery({ query }));
    this.dialogRef.close();
  }
}
