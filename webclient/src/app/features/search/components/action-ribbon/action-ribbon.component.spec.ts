import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ActionRibbonComponent } from './action-ribbon.component';

describe('ActionRibbonComponent', () => {
  let component: ActionRibbonComponent;
  let fixture: ComponentFixture<ActionRibbonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ActionRibbonComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ActionRibbonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
