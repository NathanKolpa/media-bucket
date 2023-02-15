import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'bytes'
})
export class BytesPipe implements PipeTransform {

  transform(bytes: number): unknown {

    // From https://stackoverflow.com/questions/10420352/converting-file-size-in-bytes-to-human-readable-string

    const thresh = 1000;
    const decimalPlace = 1;

    if (Math.abs(bytes) < thresh) {
      return bytes + ' B';
    }


    const units = ['kB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    let u = -1;
    const r = 10 ** decimalPlace;

    do {
      bytes /= thresh;
      ++u;
    } while (Math.round(Math.abs(bytes) * r) / r >= thresh && u < units.length - 1);

    return bytes.toFixed(decimalPlace) + ' ' + units[u];
  }

}
