import {Component, EventEmitter, Output} from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {ChartSeriesQuery, PostSearchQuery} from "@core/models";

@Component({
  selector: 'app-query-form',
  templateUrl: './query-form.component.html',
  styleUrls: ['./query-form.component.scss']
})
export class QueryFormComponent {
  form = new FormGroup({
    name: new FormControl('', [Validators.required])
  })

  @Output()
  public querySubmit = new EventEmitter<ChartSeriesQuery>();

  submitForm() {
    if (!this.form.valid) {
      return;
    }

    let name = this.form.controls.name.value!;

    this.querySubmit.emit(new ChartSeriesQuery(name, PostSearchQuery.empty(), 'count'));
  }
}
