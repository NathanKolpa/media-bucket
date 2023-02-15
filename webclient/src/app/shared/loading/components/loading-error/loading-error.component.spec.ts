import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LoadingErrorComponent } from './loading-error.component';

describe('LoadingErrorComponent', () => {
  let component: LoadingErrorComponent;
  let fixture: ComponentFixture<LoadingErrorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ LoadingErrorComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(LoadingErrorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
