import { ComponentFixture, TestBed } from '@angular/core/testing';

import { QueryEditModalComponent } from './query-edit-modal.component';

describe('QueryEditModalComponent', () => {
  let component: QueryEditModalComponent;
  let fixture: ComponentFixture<QueryEditModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ QueryEditModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(QueryEditModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
