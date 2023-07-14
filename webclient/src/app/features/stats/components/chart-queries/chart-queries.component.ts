import {Component, EventEmitter, Input, OnDestroy, Output} from '@angular/core';
import {ChartDiscriminator, ChartDiscriminatorDuration, ChartQuery, ChartSeriesQuery} from "@core/models";
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {skip, Subscription, tap} from "rxjs";

@Component({
  selector: 'app-chart-queries',
  templateUrl: './chart-queries.component.html',
  styleUrls: ['./chart-queries.component.scss']
})
export class ChartQueriesComponent implements OnDestroy {
  @Output()
  public addQuery = new EventEmitter();

  public series: ChartSeriesQuery[] = [];

  @Input()
  public set query(value: ChartQuery | null) {
    if (value == null) {
      this.form.reset();
      return;
    }

    this.series = value.series;

    if (this.form.controls.title.value != value.name) {
      this.form.controls.title.setValue(value.name);
    }

    if (this.form.controls.discriminator.value != value.discriminator.discriminator) {
      this.form.controls.discriminator.setValue(value.discriminator.discriminator);
    }
  }

  @Output()
  public titleChanged = new EventEmitter<string | null>();

  @Output()
  public discriminatorChanged = new EventEmitter<ChartDiscriminator>();

  @Output()
  public load = new EventEmitter();

  @Output()
  public removeSeries = new EventEmitter<number>();

  form = new FormGroup({
    title: new FormControl<string | null>(null),
    discriminator: new FormControl('none', [Validators.required]),
    duration: new FormControl<null | ChartDiscriminatorDuration>(null)
  });

  private titleSub: Subscription;
  private disSub: Subscription;
  private durationSub: Subscription;

  constructor() {
    this.titleSub = this.form.controls.title.valueChanges.pipe(skip(1)).subscribe(x => this.titleChanged.emit(x));
    this.disSub = this.form.controls.discriminator.valueChanges.pipe(
      tap(x => {
        if (x == 'duration') {
          this.form.controls.duration.setValidators([Validators.required])
        } else {
          this.form.controls.duration.clearValidators();
        }
      }),
      skip(1)).subscribe(x => {
        if (x == 'none') {
          this.discriminatorChanged.emit(new ChartDiscriminator('none', null));
        }
        else if (x == 'duration' && this.form.controls.duration.value) {
          this.discriminatorChanged.emit(new ChartDiscriminator('duration', this.form.controls.duration.value))
        }
    });

    this.durationSub = this.form.controls.duration.valueChanges.subscribe(x => {
      if (this.form.controls.discriminator.value == 'duration' && x !== null) {
        this.discriminatorChanged.emit(new ChartDiscriminator('duration', x))
      }
    })
  }

  public get invalidQuery(): boolean {
    return this.form.invalid || this.series.length == 0
  }

  ngOnDestroy(): void {
    this.titleSub.unsubscribe();
    this.disSub.unsubscribe();
    this.durationSub.unsubscribe();
  }
}
