import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output, ViewChild} from '@angular/core';
import {SearchPost} from "@core/models";
import {MatRipple} from "@angular/material/core";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-listing',
  templateUrl: './listing.component.html',
  styleUrls: ['./listing.component.scss']
})
export class ListingComponent {

  disableCardRipple = false;

  @Input()
  public post: SearchPost | null = null;

  @Input()
  public height: number = 100;

  @Output()
  public showInfo = new EventEmitter<SearchPost>();

  @Output()
  public showDetail = new EventEmitter<SearchPost>();
}
