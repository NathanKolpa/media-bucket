import { AfterViewInit, ChangeDetectionStrategy, Component, Input, ViewChild } from '@angular/core';
import { Media } from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-pdf-viewer',
  templateUrl: './pdf-viewer.component.html',
  styleUrls: ['./pdf-viewer.component.scss']
})
export class PdfViewerComponent implements AfterViewInit {
  @ViewChild('pdfViewer', { static: false })
  public pdfViewer?: PdfViewerComponent;
  public zoom = 100;
  @Input()
  public className: string | null = null;

  public _media: Media | null = null;

  @Input()
  public set media(value: Media | null) {
    this._media = value;

    if (this.pdfViewer) {
      this.updateViewer();
    }
  }

  ngAfterViewInit(): void {
    this.updateViewer();
  }

  private updateViewer() {
    if (!this.pdfViewer) {
      return;
    }

    if (this._media) {
      console.log(this._media.shareUrl);
      (this.pdfViewer as any).pdfSrc = encodeURIComponent(this._media.shareUrl);
      (this.pdfViewer as any).refresh();
    }
  }
}
