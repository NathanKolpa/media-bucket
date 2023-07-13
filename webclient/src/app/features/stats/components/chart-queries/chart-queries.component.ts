import {Component, EventEmitter, Input, Output} from '@angular/core';
import {ChartsQuery} from "@core/models";

@Component({
  selector: 'app-chart-queries',
  templateUrl: './chart-queries.component.html',
  styleUrls: ['./chart-queries.component.scss']
})
export class ChartQueriesComponent {
  @Input()
  public title: string | null = null;

  @Output()
  public titleChange = new EventEmitter<string | null>();

  @Output()
  public addQuery = new EventEmitter();

  @Input()
  public queries: ChartsQuery[] = [];

  @Output()
  public load = new EventEmitter();
}
