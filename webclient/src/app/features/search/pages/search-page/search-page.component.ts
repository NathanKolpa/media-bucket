import { ChangeDetectionStrategy, Component, HostListener, OnDestroy } from '@angular/core';
import { fromSearch, searchActions } from '@features/search/store';
import { Store } from "@ngrx/store";
import { Post, PostSearchQuery, SearchPost, SelectedBucket, Tag } from "@core/models";
import { fromBucket } from '@features/bucket/store';
import { MatDialog } from "@angular/material/dialog";
import { ConfirmComponent } from "@core/services/confirm/confirm.guard";
import { filter, first, Subscription, tap } from "rxjs";
import { EditPostRequest } from "@features/search/components/post-detail-sidebar/post-detail-sidebar.component";
import { Listing } from "@core/models/listing";
import { ApiService } from '@core/services';
import { Clipboard } from '@angular/cdk/clipboard';

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-search-page',
  templateUrl: './search-page.component.html',
  styleUrls: ['./search-page.component.scss']
})
export class SearchPageComponent implements OnDestroy, ConfirmComponent {

  public bucket$ = this.store.select(fromBucket.selectBucket);
  posts$ = this.store.select(fromSearch.selectPosts);
  postCount$ = this.store.select(fromSearch.selectPostCount);
  postsLoadingState$ = this.store.select(fromSearch.selectNextPageLoadingState);
  showSidebar$ = this.store.select(fromSearch.selectShowSidebar);
  activeJobs$ = this.store.select(fromSearch.selectActiveUploadJobs);
  sidebarPost$ = this.store.select(fromSearch.selectSidebarPost);
  sidebarPostLoadingState$ = this.store.select(fromSearch.selectSidebarPostLoadingState);
  searchTags$ = this.store.select(fromSearch.selectSearchTags);
  searchQuery$ = this.store.select(fromSearch.selectSearchQuery);
  private hasUnsavedInput = false;
  private unsavedInputSub: Subscription;
  private leaveMessage = 'You have unsaved progress on this page, are you sure you want to leave?';

  constructor(private store: Store, private dialog: MatDialog, private clipboard: Clipboard, private api: ApiService) {

    this.unsavedInputSub = this.activeJobs$.subscribe(activeJobs => {
      this.hasUnsavedInput = activeJobs > 0;
    });

    this.bucket$.pipe(filter(x => x !== null), first()).subscribe(bucket => {
      if (bucket !== null) {
        this.loadNext(bucket);
      }
    })
  }

  castPostToListing(posts: SearchPost[]): Listing[] {
    return posts as Listing[];
  }

  loadNext(bucket: SelectedBucket) {
    this.store.dispatch(searchActions.loadNext({ bucket }));
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
    this.store.dispatch(searchActions.showPostSidebar({ bucket, postId: post.id }));
  }

  showPost(bucket: SelectedBucket, post: SearchPost) {
    this.store.dispatch(searchActions.showPost({ bucket, postId: post.id, showPopup: true }));
  }

  showUploadDialog() {
    this.store.dispatch(searchActions.showUploadDialog());
  }

  showUploadProgress() {
    this.store.dispatch(searchActions.showUploadProgress());
  }

  deletePost(bucket: SelectedBucket, post: Post) {
    this.store.dispatch(searchActions.requestPostDelete({ post, bucket }));
  }

  showNavigationWarning(): boolean | string {
    return this.hasUnsavedInput ? this.leaveMessage : false;
  }

  @HostListener('window:beforeunload', ['$event'])
  beforeUnloadHandler(event: any) {
    if (this.hasUnsavedInput) {
      event.returnValue = this.leaveMessage;
    }
  }

  openManageTags(bucket: SelectedBucket) {
    this.store.dispatch(searchActions.openManageTags({ bucket }));
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

  editPost(bucket: SelectedBucket, req: EditPostRequest) {
    this.store.dispatch(searchActions.updatePost({
      bucket,
      title: req.title,
      description: req.description,
      source: req.source,
      postId: req.postId,
      tags: req.tags
    }))
  }

  copyQuery(bucket: SelectedBucket, query: PostSearchQuery) {
    let str = this.api.searchQueryToPlaylistUrl(bucket.auth, query);
    this.clipboard.copy(str);
  }
}
