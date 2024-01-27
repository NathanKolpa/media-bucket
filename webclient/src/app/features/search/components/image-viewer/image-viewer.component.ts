import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Media} from "@core/models";

interface Transformation {
  originX: number,
  originY: number,
  translateX: number,
  translateY: number,
  scale: number
}

const initialTransform: Transformation = {
  originX: 0,
  originY: 0,
  translateX: 0,
  translateY: 0,
  scale: 1
}

// zooming from https://betterprogramming.pub/implementation-of-zoom-and-pan-in-69-lines-of-javascript-8b0cb5f221c1
// pinch controls from https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events/Pinch_zoom_gestures
@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-image-viewer',
  templateUrl: './image-viewer.component.html',
  styleUrls: ['./image-viewer.component.scss']
})
export class ImageViewerComponent {
  @Output()
  public loadingError = new EventEmitter<string>();

  @Output()
  public viewChanged = new EventEmitter();
  @Output()
  public originalSizeChange = new EventEmitter<boolean>();
  @Input()
  public className: string | null = null;
  private pointerStack: PointerEvent[] = [];
  private pointerZoomDiff: number | null = null;
  private pointerX: number | null = null;
  private pointerY: number | null = null;
  private minScale = 0.1;
  private maxScale = 30;
  private scaleSensitivity = 10;
  private transformation = initialTransform;

  public _media: Media | null = null;

  @Input()
  public set media(value: Media | null) {
    this._media = value;
    this.resetTransform();
  }

  private _originalSize = true;

  @Input()
  public set originalSize(value: boolean) {
    if (value && !this._originalSize) {
      this.resetTransform();
    }

    this._originalSize = value;
  }

  resetTransform() {
    this.transformation = initialTransform;
  }

  mouseWheel(element: HTMLImageElement, event: WheelEvent) {
    event.preventDefault();
    this.zoom(element, Math.sign(event.deltaY) > 0 ? -1 : 1, event.pageX, event.pageY);
  }

  mouseMove(event: PointerEvent) {
    if (event.buttons != 1 || event.pointerType != 'mouse') {
      return;
    }

    event.preventDefault();
    this.pan(event.movementX, event.movementY);
  }

  public getCss(): object {
    return {
      transformOrigin: `${this.transformation.originX}px ${this.transformation.originY}px`,
      transform: `matrix(${this.transformation.scale}, 0, 0, ${this.transformation.scale}, ${this.transformation.translateX}, ${this.transformation.translateY})`
    }
  }

  private pan(originX: number, originY: number) {
    this.transformation = {
      translateX: this.transformation.translateX + originX,
      translateY: this.transformation.translateY + originY,
      originX: this.transformation.originX,
      originY: this.transformation.originY,
      scale: this.transformation.scale,
    }

    this.emitChange();
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

    this.emitChange();
  }

  private emitChange() {
    if (this._originalSize) {
      this.originalSizeChange.emit(false);
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
}
