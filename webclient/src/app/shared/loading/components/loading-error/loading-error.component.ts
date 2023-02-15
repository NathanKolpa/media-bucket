import {Component, EventEmitter, Input, Output} from '@angular/core';
import {Failure} from "@core/models";

@Component({
  selector: 'app-loading-error',
  templateUrl: './loading-error.component.html',
  styleUrls: ['./loading-error.component.scss']
})
export class LoadingErrorComponent {
  @Input()
  public failure: Failure | null = null;

  @Output()
  public loadAgain = new EventEmitter();

  loadAgainClicked = false;

  clickLoadAgain() {
    this.loadAgainClicked = true;
    this.loadAgain.emit();
  }
}
