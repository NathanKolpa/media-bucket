import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from '@angular/core';
import { Listing } from "@core/models/listing";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-listing',
  templateUrl: './listing.component.html',
  styleUrls: ['./listing.component.scss']
})
export class ListingComponent {
  public hasThumbnailError = false;
  private _item: Listing | null = null;

  get item(): Listing | null {
    return this._item;
  }

  @Input()
  set item(value: Listing | null) {
    this._item = value;
    this.hasThumbnailError = false;
  }

  disableCardRipple = false;

  @Input()
  public queryParams: any | null = null;

  @Input()
  public height: number = 100;

  @Output()
  public showInfo = new EventEmitter();

  @Output()
  public showDetail = new EventEmitter();

  @Input()
  public disableFooter = false;

  handleThumbnailError() {
    this.hasThumbnailError = true;
  }
}
