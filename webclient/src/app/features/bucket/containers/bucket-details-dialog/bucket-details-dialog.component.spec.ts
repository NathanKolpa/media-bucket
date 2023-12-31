import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BucketDetailsDialogComponent } from './bucket-details-dialog.component';

describe('BucketDetailsDialogComponent', () => {
  let component: BucketDetailsDialogComponent;
  let fixture: ComponentFixture<BucketDetailsDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BucketDetailsDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BucketDetailsDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
