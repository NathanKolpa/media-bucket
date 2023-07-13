import {
  AfterViewInit,
  ChangeDetectionStrategy,
  Component,
  ElementRef,
  EventEmitter,
  Input,
  OnDestroy,
  Output,
  ViewChild
} from '@angular/core';
import {Media} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-video-player',
  templateUrl: './video-player.component.html',
  styleUrls: ['./video-player.component.scss']
})
export class VideoPlayerComponent implements AfterViewInit, OnDestroy {

  @ViewChild('video', {static: false})
  public videoPlayer?: ElementRef<HTMLVideoElement>;
  @Input()
  public className: string | null = null;
  @Output()
  public nextVideo = new EventEmitter();

  public _media: Media | null = null;

  @Input()
  public set media(value: Media | null) {
    this._media = value;

    if (this.videoPlayer) {
      this.updateVideoPlayer();
    }
  }

  ngAfterViewInit(): void {
    this.updateVideoPlayer();
  }

  ngOnDestroy(): void {
    if (this.videoPlayer) {
      this.videoPlayer.nativeElement.pause()
      this.videoPlayer.nativeElement.removeAttribute('src');
      this.videoPlayer.nativeElement.load();
    }
  }

  private updateVideoPlayer() {
    if (!this.videoPlayer) {
      return;
    }

    let element = this.videoPlayer.nativeElement;

    element.src = this._media?.url ?? '';
    element.load();

    element.onended = () => {
      if (element.duration >= 10) {
        this.nextVideo.emit();
      } else {
        element.currentTime = 0;
        let _ = element.play();
      }
    }

    let _ = element.play()
  }
}
