import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from '@angular/core';
import { PostSearchQuery, Tag } from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-tag',
  templateUrl: './tag.component.html',
  styleUrls: ['./tag.component.scss']
})
export class TagComponent {
  @Input()
  public tag: Tag | null = null;

  public get searchParams(): any {
    if (this.searchQuery === null || this.tag === null) {
      return {};
    }

    return this.searchQuery.addTag(this.tag).queryParams();
  }

  @Input()
  public searchQuery: PostSearchQuery | null = null;

  @Input()
  public showDelete = false;

  @Input()
  public searchOnClick = false;

  @Output()
  public removeTag = new EventEmitter();

  @Output()
  public clicked = new EventEmitter();
}
