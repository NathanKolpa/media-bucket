import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from '@angular/core';
import { LoadingState, PostDetail, SelectedBucket, Tag } from "@core/models";
import { FormControl, FormGroup } from "@angular/forms";

export interface EditPostRequest {
  postId: number
  title: string | null,
  description: string | null,
  source: string | null,
  tags: Tag[]
}

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-post-detail-sidebar',
  templateUrl: './post-detail-sidebar.component.html',
  styleUrls: ['./post-detail-sidebar.component.scss']
})
export class PostDetailSidebarComponent {
  @Output()
  public editTag = new EventEmitter<Tag>();
  @Input()
  public open = false;
  @Output()
  public searchForTag = new EventEmitter<Tag>();
  @Output()
  public postEditSubmit = new EventEmitter<EditPostRequest>();
  @Input()
  public bucket: SelectedBucket | null = null;
  @Input()
  public loadingState: LoadingState | null = null;
  form = new FormGroup({
    title: new FormControl<string | null>(null),
    description: new FormControl<string | null>(null),
    source: new FormControl<string | null>(null),
  });
  postTags: Tag[] = [];

  private _post: PostDetail | null = null;

  get post(): PostDetail | null {
    return this._post;
  }

  @Input()
  set post(value: PostDetail | null) {
    this._post = value;

    this.form.controls.title.setValue(value?.title ?? null);
    this.form.controls.description.setValue(value?.description ?? null);
    this.form.controls.source.setValue(value?.source ?? null);
    this.postTags = value?.tags ?? [];
  }

  submit() {
    if (this.form.valid && this._post) {
      this.postEditSubmit.emit({
        title: this.form.controls.title.value,
        description: this.form.controls.description.value,
        source: this.form.controls.source.value,
        tags: this.postTags,
        postId: this._post.id
      })
    }
  }
}
