import {Component, ElementRef, EventEmitter, Input, Output, ViewChild} from '@angular/core';
import {Media} from "@core/models";

@Component({
  selector: 'app-pdf-viewer',
  templateUrl: './pdf-viewer.component.html',
  styleUrls: ['./pdf-viewer.component.scss']
})
export class PdfViewerComponent {
  @ViewChild('pdfViewer', {static: false})
  public pdfViewer?: PdfViewerComponent;

  public _media: Media | null = null;

  @Input()
  public set media(value: Media | null) {
    this._media = value;

    if (this.pdfViewer) {
      this.updateViewer();
    }
  }

  @Input()
  public className: string | null = null;


  ngAfterViewInit(): void {
    this.updateViewer();
  }

  private updateViewer() {
    if (!this.pdfViewer) {
      return;
    }

    if(this._media) {
      (this.pdfViewer as any).pdfSrc = encodeURIComponent(this._media.url);
      (this.pdfViewer as any).refresh();
    }
  }
}
