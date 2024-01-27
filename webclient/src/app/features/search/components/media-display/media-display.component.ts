import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Media} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-media-display',
  templateUrl: './media-display.component.html',
  styleUrls: ['./media-display.component.scss']
})
export class MediaDisplayComponent {
  private _media: Media | null = null;

  get media(): Media | null {
    return this._media;
  }

  @Input()
  set media(value: Media | null) {
    this._media = value;
    this.loadingError = null;
  }

  @Output()
  public nextItem = new EventEmitter();


  @Input()
  public className: string | null = null;

  @Input()
  public originalSize = true;

  @Output()
  public originalSizeChange = new EventEmitter<boolean>();

  public loadingError: string | null = null;

  handleLoadingError(message: string) {
    console.log(message)
    this.loadingError = message;
  }

  retryLoad() {
    this.loadingError = null;
  }
}
