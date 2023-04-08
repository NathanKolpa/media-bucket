import {ChangeDetectionStrategy, Component, Input} from '@angular/core';
import {Media} from "@core/models";

interface Transformation {
  originX: number,
  originY: number,
  translateX: number,
  translateY: number,
  scale: number
}

// from https://betterprogramming.pub/implementation-of-zoom-and-pan-in-69-lines-of-javascript-8b0cb5f221c1
@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-image-viewer',
  templateUrl: './image-viewer.component.html',
  styleUrls: ['./image-viewer.component.scss']
})
export class ImageViewerComponent {
  public minScale = 0.1;
  public maxScale = 30;
  public scaleSensitivity = 10;
  public transformation: Transformation = {
    originX: 0,
    originY: 0,
    translateX: 0,
    translateY: 0,
    scale: 1
  };

  @Input()
  public media: Media | null = null;

  @Input()
  public className: string | null = null;

  mouseWheel(element: HTMLImageElement, event: WheelEvent) {
    event.preventDefault();
    this.zoom(element, Math.sign(event.deltaY) > 0 ? -1 : 1, event.pageX, event.pageY);
  }

  mouseMove(_element: HTMLImageElement, event: MouseEvent) {
    if (event.buttons != 1) {
      return;
    }

    event.preventDefault();
    this.pan(event.movementX, event.movementY);
  }

  private pan(originX: number, originY: number) {
    this.transformation.translateX += originX;
    this.transformation.translateY += originY;
  }

  private zoom(element: HTMLImageElement, deltaScale: number, x: number, y: number) {
    const {left, top} = element.getBoundingClientRect();
    const newScale = this.getScale(deltaScale);

    const originX = x - left;
    const originY = y - top;
    const newOriginX = originX / this.transformation.scale;
    const newOriginY = originY / this.transformation.scale;

    const translateX = this.getTranslate(originX, this.transformation.originX, this.transformation.translateX);
    const translateY = this.getTranslate(originY, this.transformation.originY, this.transformation.translateY);

    this.transformation = {
      originX: newOriginX, originY: newOriginY, translateX, translateY, scale: newScale
    }
  }

  private getScale(deltaScale: number): number {
    let newScale = this.transformation.scale + (deltaScale / (this.scaleSensitivity / this.transformation.scale));
    newScale = Math.max(this.minScale, Math.min(newScale, this.maxScale));
    return newScale;
  }

  private inRange(minScale: number, maxScale: number, scale: number): boolean {
    return scale <= maxScale && scale >= minScale;
  }

  private getTranslate(pos: number, prevPos: number, translate: number) {
    return this.inRange(this.minScale, this.maxScale, this.transformation.scale) && pos !== prevPos
      ? translate + (pos - prevPos * this.transformation.scale) * (1 - 1 / this.transformation.scale)
      : translate;
  }

  public getCss(): object {
    return {
      transformOrigin: `${this.transformation.originX}px ${this.transformation.originY}px`,
      transform: `matrix(${this.transformation.scale}, 0, 0, ${this.transformation.scale}, ${this.transformation.translateX}, ${this.transformation.translateY})`
    }
  }
}
