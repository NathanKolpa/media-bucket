import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LoginTileComponent } from './login-tile.component';

describe('LoginTileComponent', () => {
  let component: LoginTileComponent;
  let fixture: ComponentFixture<LoginTileComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ LoginTileComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(LoginTileComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
