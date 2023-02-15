import {Component, EventEmitter, Input, Output} from '@angular/core';
import {LoadingState} from "@core/models";

@Component({
  selector: 'app-loadable-content',
  templateUrl: './loadable-content.component.html',
  styleUrls: ['./loadable-content.component.scss']
})
export class LoadableContentComponent {
  private _loadingState: LoadingState | null = null;
  public showLoading = false;
  private loadingTimeout: any = null;

  @Input()
  public showLoadingTimeout = 100;

  get loadingState(): LoadingState | null {
    return this._loadingState;
  }

  @Input()
  set loadingState(value: LoadingState | null) {
    this._loadingState = value;

    if (this.loadingTimeout !== null) {
      clearTimeout(this.loadingTimeout);
    }

    if (value?.isLoading) {
      this.loadingTimeout = setTimeout(() => this.showLoading = true, this.showLoadingTimeout);
    }
    else {
      this.showLoading = false;
    }
  }

  @Output()
  public loadAgain = new EventEmitter();
}
