import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfirmDeletePostDialogComponent } from './confirm-delete-post-dialog.component';

describe('ConfirmDeletePostDialogComponent', () => {
  let component: ConfirmDeletePostDialogComponent;
  let fixture: ComponentFixture<ConfirmDeletePostDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ConfirmDeletePostDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfirmDeletePostDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
