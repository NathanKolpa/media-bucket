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
  private pointerStack: PointerEvent[] = [];
  private pointerZoomDiff: number | null = null;
  private pointerX: number | null = null;
  private pointerY: number | null = null;

  private minScale = 0.1;
  private maxScale = 30;
  private scaleSensitivity = 10;
  private transformation: Transformation = {
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

  private touchEnabled = true;

  enableTouch() {
    this.touchEnabled = true;
  }

  disableTouch() {
    this.touchEnabled = false;
  }

  pointerDown(event: PointerEvent) {
    if (!this.touchEnabled) {
      return;
    }

    this.pointerStack.push(event);
  }

  pointerUp(event: PointerEvent) {
    if (!this.touchEnabled) {
      return;
    }

    const index = this.pointerStack.findIndex((cachedEv) => cachedEv.pointerId === event.pointerId);
    this.pointerStack.splice(index, 1);

    if (this.pointerStack.length < 1) {
      this.pointerX = null;
      this.pointerY = null;
    } else if (this.pointerStack.length < 2) {
      this.pointerZoomDiff = null;
    }
  }

  pointerMove(element: HTMLImageElement, event: PointerEvent) {
    if (!this.touchEnabled) {
      return;
    }

    const index = this.pointerStack.findIndex((cachedEv) => cachedEv.pointerId === event.pointerId);
    this.pointerStack[index] = event;

    if (this.pointerStack.length == 2) {
      const [pointerA, pointerB] = this.pointerStack;
      const curDiff = Math.sqrt(Math.pow(pointerB.clientX - pointerA.clientX, 2) + Math.pow(pointerB.clientY - pointerA.clientY, 2));

      if (this.pointerZoomDiff != null) {
        this.zoom(element, (curDiff - this.pointerZoomDiff) / this.scaleSensitivity, event.pageX, event.pageY);
      }

      this.pointerZoomDiff = curDiff;
    } else if (this.pointerStack.length == 1) {

      if (this.pointerX != null && this.pointerY != null) {
        this.pan(event.movementX - this.pointerX, event.movementY - this.pointerY);
      }

      this.pointerX = event.movementX;
      this.pointerY = event.movementY;
    }
  }

  mouseWheel(element: HTMLImageElement, event: WheelEvent) {
    if (!event.shiftKey) {
      return;
    }

    event.preventDefault();
    this.zoom(element, Math.sign(event.deltaY) > 0 ? -1 : 1, event.pageX, event.pageY);
  }

  mouseMove(event: MouseEvent) {
    if (event.buttons != 1 || !event.shiftKey) {
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
