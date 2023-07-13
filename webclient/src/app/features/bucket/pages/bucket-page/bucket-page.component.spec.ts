import {ComponentFixture, TestBed} from '@angular/core/testing';

import {BucketPageComponent} from './bucket-page.component';

describe('BucketLayoutComponent', () => {
  let component: BucketPageComponent;
  let fixture: ComponentFixture<BucketPageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [BucketPageComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(BucketPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
