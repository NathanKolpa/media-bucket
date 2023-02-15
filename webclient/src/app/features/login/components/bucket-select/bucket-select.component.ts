import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {AuthenticatedBucket} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-bucket-select',
  templateUrl: './bucket-select.component.html',
  styleUrls: ['./bucket-select.component.scss']
})
export class BucketSelectComponent {
  @Input()
  public buckets: AuthenticatedBucket[] = [];

  @Input()
  public selectedId = 0;

  @Output()
  public selectedIdChange = new EventEmitter<number>();

  public setSelected(id: number) {
    this.selectedIdChange.emit(id)
  }
}
