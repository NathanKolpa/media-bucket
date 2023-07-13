import {ComponentFixture, TestBed} from '@angular/core/testing';

import {ChartQueriesComponent} from './chart-queries.component';

describe('ChartQueriesComponent', () => {
  let component: ChartQueriesComponent;
  let fixture: ComponentFixture<ChartQueriesComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ChartQueriesComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(ChartQueriesComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
