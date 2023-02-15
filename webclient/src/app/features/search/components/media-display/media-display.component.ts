import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Media} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-media-display',
  templateUrl: './media-display.component.html',
  styleUrls: ['./media-display.component.scss']
})
export class MediaDisplayComponent {

  @Output()
  public nextItem = new EventEmitter();

  @Input()
  public media: Media | null = null;

  @Input()
  public className: string | null = null;

  encodeURIComponent(string: string): string {
    return encodeURIComponent(string);
  }
}
