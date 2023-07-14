import {Component} from '@angular/core';
import {MatDialogRef} from "@angular/material/dialog";
import {Store} from "@ngrx/store";
import {ChartSeriesQuery, PageParams, Tag} from "@core/models";
import {statsActions} from '@features/stats/store';
import {ApiService} from "@core/services";
import { fromBucket } from '@features/bucket/store';
import {combineAll, combineLatestAll, combineLatestWith, filter, map, Observable, Subject, switchMap} from "rxjs";

@Component({
  selector: 'app-query-add-modal',
  templateUrl: './query-add-modal.component.html',
  styleUrls: ['./query-add-modal.component.scss']
})
export class QueryAddModalComponent {
  public tags$: Observable<Tag[]>;

  private bucket$ = this.store.select(fromBucket.selectBucket);
  private query$ = new Subject<string | null>();

  constructor(private dialogRef: MatDialogRef<QueryAddModalComponent>, private store: Store, private api: ApiService) {
    this.tags$ = this.bucket$.pipe(
      combineLatestWith(this.query$),
      filter(([bucket]) => bucket !== null),
      switchMap(([bucket, query]) => this.api.searchTags(bucket!.auth, new PageParams(25, 0), query ?? '')),
      map(x => x.tags)
    )
  }

  addSeriesQuery(query: ChartSeriesQuery) {
    this.store.dispatch(statsActions.addSeriesQuery({query}));
    this.dialogRef.close();
  }

  searchQueryChange(text: string | null) {
    this.query$.next(text);
  }
}
