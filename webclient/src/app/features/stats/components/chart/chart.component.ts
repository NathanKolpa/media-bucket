import {Component, Input} from '@angular/core';
import {Chart, ChartSeries} from "@core/models";
import {ChartConfiguration, ChartDataset, ChartOptions} from "chart.js";

@Component({
  selector: 'app-chart',
  templateUrl: './chart.component.html',
  styleUrls: ['./chart.component.scss']
})
export class ChartComponent {

  public options: any = {};
  public chartData: any = null;
  public type: any = null;

  @Input()
  public set chart(chart: Chart | null) {
    if (chart === null || chart.series.length <= 0) {
      this.chartData = null;
      return;
    }

    this.options.responsive = true;

    this.options.interaction = {
      mode: 'index',
      intersect: false,
    };

    this.options.plugins = {
      title: {
        text: chart.name == null ? [] : chart.name,
        display: chart.name !== null
      }
    }

    if (chart.discriminator.discriminator == 'duration' && chart.discriminator.duration !== null) {
      this.options.scales = {
        xAxes: {
          type: 'time',
          time: {
            unit: chart.discriminator.duration
          }
        }
      };
    }

    if (chart.discriminator.discriminator == 'duration') {
      this.type = 'line';
      this.chartData = {
        datasets: chart.series.map(x => this.lineSeries(x))
      }
    }
    else if (chart.discriminator.discriminator == 'none') {
      this.type = 'bar';
      this.chartData = this.barChart(chart);
    }
  }

  private lineSeries(chart: ChartSeries): ChartDataset<"line", any[]> {
    return {
      label: chart.name,
      data: chart.points.map((point, i) => {
        switch (point.type) {
          case 'none':
            return {y: point.y, x: i}
          case "time":
            return {y: point.y, x: point.x}
        }
      })
    }
  }

  private barChart(chart: Chart): any {
    return {
      labels: chart.series.map(x => x.name),
      datasets: [
        {
          label: 'Count',
          data: chart.series.map(x => x.points[0].y)
        }
      ]
    }
  }
}
