import {ChangeDetectionStrategy, Component} from '@angular/core';
import {fromBucket} from '@features/bucket/store';
import {Store} from "@ngrx/store";
import {SelectedBucket, Tag} from "@core/models";
import {fromSearch, searchActions} from '@features/search/store';

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-manage-tags-dialog',
  templateUrl: './manage-tags-dialog.component.html',
  styleUrls: ['./manage-tags-dialog.component.scss']
})
export class ManageTagsDialogComponent {
  bucket$ = this.store.select(fromBucket.selectBucket);
  searchLoadingState$ = this.store.select(fromSearch.selectTagEditSearchLoadingState);
  searchResultCount$ = this.store.select(fromSearch.selectTagEditSearchTagCount);
  searchResults$ = this.store.select(fromSearch.selectTagEditSearchTags);

  selectedTagLoadingState$ = this.store.select(fromSearch.selectTagEditSelectedTagLoadingState);
  selectedTag$ = this.store.select(fromSearch.selectTagEditSelectedTag);


  lastTagId: number | null = null;

  changeSearchQuery(bucket: SelectedBucket, query: string | null) {
    this.store.dispatch(searchActions.tagEditSearchQueryChange({query, bucket}));
  }

  loadNext(bucket: SelectedBucket) {
    this.store.dispatch(searchActions.loadTagEditNextSearchTags({bucket}))
  }

  changeSelectedTag(bucket: SelectedBucket, tagId: number | null) {
    if (tagId !== null) {
      this.store.dispatch(searchActions.tagEditSelectTag({bucket, tagId}))
      this.lastTagId = tagId;
    } else {
      this.store.dispatch(searchActions.tagEditClearSelected({bucket}));
      this.lastTagId = null;
    }
  }

  reloadSelectedTag(bucket: SelectedBucket) {
    if (this.lastTagId !== null) {
      this.store.dispatch(searchActions.tagEditSelectTag({bucket, tagId: this.lastTagId}))
    }
  }

  constructor(private store: Store) {
  }
}
