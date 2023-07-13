import {Component, Input} from '@angular/core';
import {Chart} from "@core/models";
import {ChartConfiguration, ChartDataset, ChartOptions, ChartType, ScatterDataPoint} from "chart.js";

@Component({
  selector: 'app-chart',
  templateUrl: './chart.component.html',
  styleUrls: ['./chart.component.scss']
})
export class ChartComponent {

  public options: ChartOptions<'line'> = {
    responsive: true,
    scales: {
      xAxes: {
        type: 'time',
        time: {
          unit: 'day'
        }
      }
    }
  }

  public chartData: ChartConfiguration<'line'>['data'] | null = null;

  @Input()
  public set charts(charts: Chart[]) {
    if (charts.length <= 0) {
      this.chartData = null;
      return;
    }

    console.log(charts.map(x => this.chartToDataset(x)))

    this.chartData = {
      datasets: charts.map(x => this.chartToDataset(x))
    }

  }

  private chartToDataset(chart: Chart):  ChartDataset<"line", any[]> {
    return {
      label: chart.name,
      data: chart.points.map((point, i) => {
        switch (point.type) {
          case 'none':
            return {y: point.y, x: i}
          case "time":
            return {y: point.y, x: point.x }
        }
      })
    }
  }
}
