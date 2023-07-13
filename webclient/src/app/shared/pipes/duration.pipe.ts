import {Pipe, PipeTransform} from '@angular/core';

@Pipe({
  name: 'duration'
})
export class DurationPipe implements PipeTransform {

  transform(value: number, ...args: unknown[]): unknown {
    let hours: string | number = Math.floor(value / 3600);
    let minutes: string | number = Math.floor((value - (hours * 3600)) / 60);
    let seconds: string | number = value - (hours * 3600) - (minutes * 60);

    let output = `${this.formatNum(minutes)}:${this.formatNum(seconds)}`;

    if (hours > 0) {
      output = hours + ':' + output;
    }

    return output;
  }

  private formatNum(value: number): string {
    return value.toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping: false})
  }
}
