import { Component, OnDestroy, ViewEncapsulation } from '@angular/core';
import { MatDialogRef } from "@angular/material/dialog";
import { delay, first, map, Subscription, withLatestFrom } from "rxjs";
import { Store } from "@ngrx/store";
import { fromSearch, searchActions } from '@features/search/store';
import { fromBucket } from '@features/bucket/store';
import { Post, PostItem, PostSearchQuery, SearchPostItem, SelectedBucket } from "@core/models";
import { AppTitleService } from "@core/services";
import { Listing } from "@core/models/listing";
import { Clipboard } from '@angular/cdk/clipboard';
import { Router } from '@angular/router';

@Component({
  encapsulation: ViewEncapsulation.None,
  selector: 'app-post-detail-dialog',
  templateUrl: './post-detail-dialog.component.html',
  styleUrls: ['./post-detail-dialog.component.scss']
})
export class PostDetailDialogComponent implements OnDestroy {
  public open = false;

  bucket$ = this.store.select(fromBucket.selectBucket);
  searchQuery$ = this.store.select(fromSearch.selectSearchQuery);
  post$ = this.store.select(fromSearch.selectViewedPost);
  currentOffset$ = this.store.select(fromSearch.selectViewOffset);
  itemLoadingState$ = this.store.select(fromSearch.selectCurrentItemLoadingState);
  postLoadingState$ = this.store.select(fromSearch.selectViewedPostLoadingState);
  item$ = this.store.select(fromSearch.selectCurrentItem);
  viewedPostMode$ = this.store.select(fromSearch.selectViewedPostMode);
  itemList$ = this.store.select(fromSearch.selectItemList);
  itemListLoadingState$ = this.store.select(fromSearch.selectItemListLoadingState);

  startingIndex$ = this.item$.pipe(map(x => x?.position ?? null));
  public originalSize = true;
  private currentBucket: SelectedBucket | null = null;
  private currentPostId: number | null = null;
  private currentOffset: number | null = null;
  private currentPosition: number = 0;
  private titleIndex: number | null = null;
  private postSubscription: Subscription;
  private bucketSubscription: Subscription;
  private itemSubscription: Subscription;
  private offsetSubscription: Subscription;

  public transformParams(query: PostSearchQuery, offset: number): any {
    return { ...query.queryParams(), focus_post: offset };
  }

  constructor(public dialogRef: MatDialogRef<PostDetailDialogComponent>, private store: Store, private title: AppTitleService, private clipboard: Clipboard, private router: Router) {
    dialogRef.afterOpened().pipe(first(), delay(0)).subscribe(() => this.open = true);

    this.dialogRef.keydownEvents()
      .pipe(withLatestFrom(this.bucket$))
      .pipe(withLatestFrom(this.currentOffset$))
      .pipe(withLatestFrom(this.searchQuery$))
      .subscribe(([[[event, bucket], offset], query]) => {
        if (bucket === null || offset === null || query == null) {
          return;
        }

        if (event.key === "Escape") {
          this.dialogRef.close();

          this.router.navigate(['buckets', bucket.bucket.id], {
            queryParams: this.transformParams(query, offset),
          });
        }
      });

    this.offsetSubscription = this.currentOffset$.subscribe(offset => {
      if (offset !== null) {
        this.currentOffset = offset;
      }
    });

    this.postSubscription = this.post$.subscribe(post => {
      if (post) {
        this.currentPostId = post.id;
      }

      if (post !== null && post.title !== null) {
        if (this.titleIndex !== null) {
          this.title.set(this.titleIndex, post.title);
        } else {
          this.titleIndex = this.title.push(post.title);
        }
      }

    });

    this.bucketSubscription = this.bucket$.subscribe(bucket => {
      if (bucket) {
        this.currentBucket = bucket;
      }
    });

    this.itemSubscription = this.item$.subscribe(item => {
      if (item) {
        this.currentPosition = item.position;
      }
    });
  }

  ngOnDestroy(): void {
    this.postSubscription.unsubscribe();
    this.itemSubscription.unsubscribe();
    this.bucketSubscription.unsubscribe();

    if (this.titleIndex !== null) {
      this.title.pop();
    }
  }

  reloadItem() {
    if (this.currentBucket && this.currentPostId && this.currentPosition) {
      this.loadItem(this.currentBucket, this.currentPostId, this.currentPosition);
    }
  }

  reloadPost() {
    if (this.currentBucket && this.currentPostId && this.currentOffset) {
      this.store.dispatch(searchActions.showPost({
        bucket: this.currentBucket,
        postId: this.currentPostId,
        offset: this.currentOffset
      }));
    }
  }

  loadItem(bucket: SelectedBucket, postId: number, position: number) {
    this.store.dispatch(searchActions.loadPostItem({
      bucket,
      position,
      postId
    }));

    this.originalSize = true;
  }

  toggleViewMode() {
    this.store.dispatch(searchActions.togglePostDetailViewMode());
    this.originalSize = true;
  }

  loadNextItems() {
    if (this.currentBucket && this.currentPostId) {
      this.store.dispatch(searchActions.loadNextPostItemList({
        bucket: this.currentBucket,
        postId: this.currentPostId
      }));
    }
  }

  loadItemFromList(bucket: SelectedBucket, postId: number, position: number) {
    this.store.dispatch(searchActions.togglePostDetailViewMode());
    this.loadItem(bucket, postId, position);
  }

  castItemsToListing(items: SearchPostItem[]): Listing[] {
    return items as Listing[];
  }

  copyItemUrl(bucket: SelectedBucket, item: PostItem) {
    let url = `${bucket.auth.base}/posts/${item.postId}/items/${item.position}/index.m3u8?token=${bucket.auth.shareToken}&include_token=true`
    this.clipboard.copy(url);
  }
}
