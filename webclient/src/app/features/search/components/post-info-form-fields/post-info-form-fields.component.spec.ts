import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PostInfoFormFieldsComponent } from './post-info-form-fields.component';

describe('PostInfoFormFieldsComponent', () => {
  let component: PostInfoFormFieldsComponent;
  let fixture: ComponentFixture<PostInfoFormFieldsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ PostInfoFormFieldsComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PostInfoFormFieldsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
