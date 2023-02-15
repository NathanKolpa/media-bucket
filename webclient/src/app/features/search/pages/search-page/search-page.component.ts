import {ChangeDetectionStrategy, Component, HostListener, OnDestroy} from '@angular/core';
import {fromSearch, searchActions} from '@features/search/store';
import {Store} from "@ngrx/store";
import {Post, PostSearchQuery, SearchPost, SelectedBucket, Tag} from "@core/models";
import {fromBucket} from '@features/bucket/store';
import {MatDialog} from "@angular/material/dialog";
import {ConfirmComponent} from "@core/services/confirm/confirm.guard";
import {Subscription} from "rxjs";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-search-page',
  templateUrl: './search-page.component.html',
  styleUrls: ['./search-page.component.scss']
})
export class SearchPageComponent implements OnDestroy, ConfirmComponent {

  private hasUnsavedInput = false;
  private unsavedInputSub: Subscription;

  public bucket$ = this.store.select(fromBucket.selectBucket);

  posts$ = this.store.select(fromSearch.selectPosts);
  postsLoadingState$ = this.store.select(fromSearch.selectNextPageLoadingState);
  showSidebar$ = this.store.select(fromSearch.selectShowSidebar);
  activeJobs$ = this.store.select(fromSearch.selectActiveUploadJobs);

  sidebarPost$ = this.store.select(fromSearch.selectSidebarPost);
  sidebarPostLoadingState$ = this.store.select(fromSearch.selectSidebarPostLoadingState);

  searchTags$ = this.store.select(fromSearch.selectSearchTags);
  searchQuery$ = this.store.select(fromSearch.selectSearchQuery);

  constructor(private store: Store, private dialog: MatDialog) {

    this.unsavedInputSub = this.activeJobs$.subscribe(activeJobs => {
      this.hasUnsavedInput = activeJobs > 0;
    })
  }

  loadNext(bucket: SelectedBucket) {
    this.store.dispatch(searchActions.loadNext({bucket}));
  }

  toggleInfo() {
    this.store.dispatch(searchActions.toggleInfo());
  }

  closeInfo() {
    this.store.dispatch(searchActions.closeInfo());
  }

  ngOnDestroy(): void {
    this.store.dispatch(searchActions.reset());
    this.unsavedInputSub.unsubscribe();
  }

  showSidebar(bucket: SelectedBucket, post: SearchPost) {
    this.store.dispatch(searchActions.showPostSidebar({bucket, postId: post.id}));
  }

  showPost(bucket: SelectedBucket, post: SearchPost) {
    this.store.dispatch(searchActions.showPost({bucket, postId: post.id, showPopup: true}));
  }

  showUploadDialog() {
    this.store.dispatch(searchActions.showUploadDialog());
  }

  showUploadProgress() {
    this.store.dispatch(searchActions.showUploadProgress());
  }

  deletePost(bucket: SelectedBucket, post: Post) {
    this.store.dispatch(searchActions.requestPostDelete({post, bucket}));
  }

  private leaveMessage = 'You have unsaved progress on this page, are you sure you want to leave?';

  showNavigationWarning(): boolean | string {
    return this.hasUnsavedInput ? this.leaveMessage : false;
  }

  @HostListener('window:beforeunload', ['$event'])
  beforeUnloadHandler(event: any) {
    if (this.hasUnsavedInput) {
      event.returnValue = this.leaveMessage;
    }
  }

  searchTextChange(bucket: SelectedBucket, query: string | null) {
    this.store.dispatch(searchActions.searchTextChange({ bucket, query }));
  }

  queryChange(bucket: SelectedBucket, query: PostSearchQuery) {
    this.store.dispatch(searchActions.searchQueryChange({ bucket, query }));
  }

  addTagToSearchQuery(bucket: SelectedBucket, tag: Tag) {
    this.store.dispatch(searchActions.addTagToSearchQuery({ bucket, tag }));
  }

}
