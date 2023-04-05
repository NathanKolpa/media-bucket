import {Component, Input} from '@angular/core';
import {Media} from "@core/models";

@Component({
  selector: 'app-image-viewer',
  templateUrl: './image-viewer.component.html',
  styleUrls: ['./image-viewer.component.scss']
})
export class ImageViewerComponent {
  @Input()
  public media: Media | null = null;

  @Input()
  public className: string | null = null;
}
