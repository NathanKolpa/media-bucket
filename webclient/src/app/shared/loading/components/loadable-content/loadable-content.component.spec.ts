import {ComponentFixture, TestBed} from '@angular/core/testing';

import {LoadableContentComponent} from './loadable-content.component';

describe('LoadableContentComponent', () => {
  let component: LoadableContentComponent;
  let fixture: ComponentFixture<LoadableContentComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [LoadableContentComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(LoadableContentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
