import {ComponentFixture, TestBed} from '@angular/core/testing';

import {BucketSelectComponent} from './bucket-select.component';

describe('BucketSelectComponent', () => {
  let component: BucketSelectComponent;
  let fixture: ComponentFixture<BucketSelectComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [BucketSelectComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(BucketSelectComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
