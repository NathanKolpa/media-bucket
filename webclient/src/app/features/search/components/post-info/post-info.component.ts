import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from '@angular/core';
import { PostDetail, PostSearchQuery, Tag } from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-post-info',
  templateUrl: './post-info.component.html',
  styleUrls: ['./post-info.component.scss']
})
export class PostInfoComponent {

  @Input()
  public post: PostDetail | null = null;


  @Input()
  public searchQuery: PostSearchQuery | null = null;

  @Output()
  public searchForTag = new EventEmitter<Tag>();
}
