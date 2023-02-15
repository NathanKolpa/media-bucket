import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Tag} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-tag',
  templateUrl: './tag.component.html',
  styleUrls: ['./tag.component.scss']
})
export class TagComponent {
  @Input()
  public tag: Tag | null = null;

  @Input()
  public showDelete = false;

  @Output()
  public removeTag = new EventEmitter();

  @Output()
  public clicked = new EventEmitter();
}
