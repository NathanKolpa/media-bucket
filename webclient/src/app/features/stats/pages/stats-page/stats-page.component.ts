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
  chartTitle$ = this.store.select(fromStats.selectTitle);
  queries$ = this.store.select(fromStats.selectQueries);
  loadingState$ = this.store.select(fromStats.selectLoadingState);
  hasChart$ = this.store.select(fromStats.selectHasChart);
  charts$ = this.store.select(fromStats.selectCharts);

  public bucket$ = this.store.select(fromBucket.selectBucket);

  constructor(private store: Store, private dialog: MatDialog, private title: AppTitleService) {
    this.title.push('Statistics');
  }

  setChartTitle(title: string | null) {
    this.store.dispatch(statsActions.updateTitle({ title }))
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
