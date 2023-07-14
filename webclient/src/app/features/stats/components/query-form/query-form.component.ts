import {Component, EventEmitter, Input, Output} from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {ChartSelect, ChartSeriesQuery, PostSearchQuery, Tag} from "@core/models";

@Component({
  selector: 'app-query-form',
  templateUrl: './query-form.component.html',
  styleUrls: ['./query-form.component.scss']
})
export class QueryFormComponent {
  form = new FormGroup({
    name: new FormControl('', [Validators.required]),
    select: new FormControl<ChartSelect>('count', [Validators.required])
  });

  public filter = PostSearchQuery.empty();

  @Output()
  public querySubmit = new EventEmitter<ChartSeriesQuery>();

  @Output()
  public searchTextChange = new EventEmitter<string | null>();

  @Input()
  public tags: Tag[] = [];

  submitForm() {
    if (!this.form.valid) {
      return;
    }

    let name = this.form.controls.name.value!;
    let select = this.form.controls.select.value!;

    this.querySubmit.emit(new ChartSeriesQuery(name, this.filter, select));
  }
}
