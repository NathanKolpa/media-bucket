import {ChangeDetectionStrategy, Component, OnDestroy} from '@angular/core';
import {fromStats, statsActions} from '@features/stats/store';
import {Store} from "@ngrx/store";
import {MatDialog} from "@angular/material/dialog";
import {QueryAddModalComponent} from "@features/stats/containers/query-add-modal/query-add-modal.component";
import {AppTitleService} from "@core/services";
import { fromBucket } from '@features/bucket/store';
import {Auth, SelectedBucket} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-stats-page',
  templateUrl: './stats-page.component.html',
  styleUrls: ['./stats-page.component.scss']
})
export class StatsPageComponent implements OnDestroy {
  query$ = this.store.select(fromStats.selectQuery);
  loadingState$ = this.store.select(fromStats.selectLoadingState);
  hasChart$ = this.store.select(fromStats.selectHasChart);
  chart$ = this.store.select(fromStats.selectChart);

  public bucket$ = this.store.select(fromBucket.selectBucket);

  constructor(private store: Store, private dialog: MatDialog, private title: AppTitleService) {
    this.title.push('Statistics');
  }

  openCreateQueryModal() {
    this.dialog.open(QueryAddModalComponent);
  }

  ngOnDestroy(): void {
    this.title.pop();
  }

  loadChart(bucket: SelectedBucket) {
    this.store.dispatch(statsActions.loadChart({ bucket }));
  }
}
