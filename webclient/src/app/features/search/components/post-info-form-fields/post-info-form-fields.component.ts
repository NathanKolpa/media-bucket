import {ChangeDetectionStrategy, Component, Input} from '@angular/core';
import {FormControl} from "@angular/forms";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-post-info-form-fields',
  templateUrl: './post-info-form-fields.component.html',
  styleUrls: ['./post-info-form-fields.component.scss']
})
export class PostInfoFormFieldsComponent {
  @Input()
  public description: FormControl | null = null;

  @Input()
  public title: FormControl | null = null;

  @Input()
  public source: FormControl | null = null;
}
