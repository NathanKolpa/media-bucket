import { ComponentFixture, TestBed } from '@angular/core/testing';

import { QueryAddModalComponent } from './query-add-modal.component';

describe('QueryAddModalComponent', () => {
  let component: QueryAddModalComponent;
  let fixture: ComponentFixture<QueryAddModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ QueryAddModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(QueryAddModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
