import {ComponentFixture, TestBed} from '@angular/core/testing';

import {PostDetailSidebarComponent} from './post-detail-sidebar.component';

describe('PostInfoContainerComponent', () => {
  let component: PostDetailSidebarComponent;
  let fixture: ComponentFixture<PostDetailSidebarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [PostDetailSidebarComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(PostDetailSidebarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
