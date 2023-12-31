import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Bucket} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.scss']
})
export class HeaderComponent {

  @Input()
  public bucket: Bucket | null = null;

  @Output()
  public logout = new EventEmitter();


  @Output()
  public showGeneralInfo = new EventEmitter();
}
