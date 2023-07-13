import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {FormControl, FormGroup} from "@angular/forms";
import {SelectedBucket, Tag, Upload} from "@core/models";
import {UploadPositionSwapEvent} from "@features/search/components/upload-list/upload-list.component";
import {Observable} from "rxjs";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-create-post-form',
  templateUrl: './create-post-form.component.html',
  styleUrls: ['./create-post-form.component.scss']
})
export class CreatePostFormComponent {
  form = new FormGroup({
    title: new FormControl<string | null>(null),
    description: new FormControl<string | null>(null),
    source: new FormControl<string | null>(null),
    flatten: new FormControl(false),
    createAnother: new FormControl()
  })

  @Input()
  bucket: SelectedBucket | null = null;

  @Input()
  tags: Tag[] = [];

  @Output()
  public tagsChange = new EventEmitter<Tag[]>();

  @Input()
  uploads: Upload[] = [];

  @Output()
  public filesAdded = new EventEmitter<File[]>();

  @Output()
  public swapUploads = new EventEmitter<UploadPositionSwapEvent>();

  @Output()
  public deleteIndexes = new EventEmitter<number[]>();
  @Output()
  public titleChange = new EventEmitter<string | null>();
  @Output()
  public sourceChange = new EventEmitter<string | null>();
  @Output()
  public descriptionChange = new EventEmitter<string | null>();
  @Output()
  public flattenChange = new EventEmitter<boolean>();

  constructor() {
    this.pipeValueChangeToEmitter(this.form.controls.title.valueChanges, this.titleChange);
    this.pipeValueChangeToEmitter(this.form.controls.source.valueChanges, this.sourceChange);
    this.pipeValueChangeToEmitter(this.form.controls.description.valueChanges, this.descriptionChange);
    this.pipeValueChangeToEmitter(this.form.controls.flatten.valueChanges, this.flattenChange);
  }

  @Input()
  public set title(value: string | null) {
    this.form.controls.title.setValue(value);
  }

  @Input()
  public set description(value: string | null) {
    this.form.controls.description.setValue(value);
  }

  @Input()
  public set source(value: string | null) {
    this.form.controls.source.setValue(value);
  }

  @Input()
  public set flatten(value: boolean) {
    this.form.controls.flatten.setValue(value);
  }

  get activeUploads(): Upload[] {
    return this.uploads.filter(x => x.state !== 'deleted');
  }

  get uploadSize(): number {
    return this.activeUploads
      .reduce((acc, upload) => acc + upload.file.size, 0)
  }

  private pipeValueChangeToEmitter(changes: Observable<any>, emitter: EventEmitter<any>) {
    changes.subscribe(x => emitter.emit(x))
  }
}
