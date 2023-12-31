import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {LoadingState, TagDetail} from "@core/models";
import {FormControl, FormGroup, Validators} from "@angular/forms";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-manage-tags-tag-edit',
  templateUrl: './manage-tags-tag-edit.component.html',
  styleUrls: ['./manage-tags-tag-edit.component.scss']
})
export class ManageTagsTagEditComponent {

  get tag(): TagDetail | null {
    return this._tag;
  }

  @Input()
  set tag(value: TagDetail | null) {
    this._tag = value;

    if(value == null) {
      return;
    }

    this.form.controls.name.setValue(value.name);
  }

  @Input()
  public detailLoadingState: LoadingState | null = null;

  private _tag: TagDetail | null = null;

  @Output()
  public reload = new EventEmitter();

  form = new FormGroup({
    name: new FormControl('', [Validators.required])
  })
}
