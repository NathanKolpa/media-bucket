import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {SearchPostItem} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-post-item-list',
  templateUrl: './post-item-list.component.html',
  styleUrls: ['./post-item-list.component.scss']
})
export class PostItemListComponent {

  @Input()
  public items: SearchPostItem[] = [];

  displayedColumns: string[] = ['position', 'display'];

  @Output()
  public clickItem = new EventEmitter<SearchPostItem>();

}
