import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ManageTagsTagEditComponent } from './manage-tags-tag-edit.component';

describe('ManageTagsTagEditComponent', () => {
  let component: ManageTagsTagEditComponent;
  let fixture: ComponentFixture<ManageTagsTagEditComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ManageTagsTagEditComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ManageTagsTagEditComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
