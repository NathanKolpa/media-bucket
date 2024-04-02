import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from '@angular/core';
import { Bucket, LoadingState, PostSearchQuery, TagDetail } from "@core/models";
import { FormControl, FormGroup, Validators } from "@angular/forms";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-manage-tags-tag-edit',
  templateUrl: './manage-tags-tag-edit.component.html',
  styleUrls: ['./manage-tags-tag-edit.component.scss']
})
export class ManageTagsTagEditComponent {
  @Output()
  public navigated = new EventEmitter();

  @Output()
  public delete = new EventEmitter<TagDetail>();

  get tag(): TagDetail | null {
    return this._tag;
  }

  @Input()
  set tag(value: TagDetail | null) {
    this._tag = value;

    if (value == null) {
      return;
    }

    this.form.controls.name.setValue(value.name);
  }

  @Input()
  public bucket: Bucket | null = null;

  @Input()
  public detailLoadingState: LoadingState | null = null;

  private _tag: TagDetail | null = null;

  @Output()
  public reload = new EventEmitter();

  form = new FormGroup({
    name: new FormControl('', [Validators.required])
  })

  public get searchQuery(): PostSearchQuery | null {
    if (this._tag == null) {
      return null;
    }

    return new PostSearchQuery([{ type: 'tag', tag: this._tag }], 'relevant', Math.random(), null);
  }
}
