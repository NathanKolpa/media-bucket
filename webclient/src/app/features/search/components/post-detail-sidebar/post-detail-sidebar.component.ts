import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {LoadingState, PostDetail, Tag} from "@core/models";
import {FormControl, FormGroup} from "@angular/forms";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-post-detail-sidebar',
  templateUrl: './post-detail-sidebar.component.html',
  styleUrls: ['./post-detail-sidebar.component.scss']
})
export class PostDetailSidebarComponent {
  private _post: PostDetail | null = null;

  @Input()
  public open = false;

  @Output()
  public searchForTag = new EventEmitter<Tag>();


  get post(): PostDetail | null {
    return this._post;
  }

  @Input()
  set post(value: PostDetail | null) {
    this._post = value;

    this.form.controls.title.setValue(value?.title ?? null);
    this.form.controls.description.setValue(value?.description ?? null);
    this.form.controls.source.setValue(value?.source ?? null);
  }

  @Input()
  public loadingState: LoadingState | null = null;

  form = new FormGroup({
    title: new FormControl<string | null>(null),
    description: new FormControl<string | null>(null),
    source: new FormControl<string | null>(null),
  })
}
