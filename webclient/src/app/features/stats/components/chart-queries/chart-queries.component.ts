import {Component, EventEmitter, Input, Output} from '@angular/core';
import {ChartQuery} from "@core/models";

@Component({
  selector: 'app-chart-queries',
  templateUrl: './chart-queries.component.html',
  styleUrls: ['./chart-queries.component.scss']
})
export class ChartQueriesComponent {
  @Output()
  public addQuery = new EventEmitter();

  @Input()
  public query: ChartQuery | null = null;

  @Output()
  public load = new EventEmitter();
}
