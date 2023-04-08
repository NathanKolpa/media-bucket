import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output, ViewChild} from '@angular/core';
import {SearchPost} from "@core/models";
import {MatRipple} from "@angular/material/core";
import {Listing} from "@core/models/listing";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-listing',
  templateUrl: './listing.component.html',
  styleUrls: ['./listing.component.scss']
})
export class ListingComponent {

  disableCardRipple = false;

  @Input()
  public item: Listing | null = null;

  @Input()
  public height: number = 100;

  @Output()
  public showInfo = new EventEmitter();

  @Output()
  public showDetail = new EventEmitter();
}
