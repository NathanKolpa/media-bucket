import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ManageTagsSearchResultsComponent } from './manage-tags-search-results.component';

describe('ManageTagsSearchResultsComponent', () => {
  let component: ManageTagsSearchResultsComponent;
  let fixture: ComponentFixture<ManageTagsSearchResultsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ManageTagsSearchResultsComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ManageTagsSearchResultsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
