import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ReloginModalComponent } from './relogin-modal.component';

describe('ReloginModalComponent', () => {
  let component: ReloginModalComponent;
  let fixture: ComponentFixture<ReloginModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ReloginModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ReloginModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
