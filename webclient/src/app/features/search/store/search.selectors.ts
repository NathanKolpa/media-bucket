import {createSelector} from "@ngrx/store";
import {selectSearchState, tagSelectors, uploadJobSelectors} from "./search.reducer";
import {PageParams} from "@core/models";

export const selectPosts = createSelector(
  selectSearchState,
  (state) => {
    return state.posts;
  }
);

export const selectTotalPosts = createSelector(
  selectSearchState,
  (state) => {
    return state.posts.length;
  }
);

export const selectJob = (id: string) => createSelector(
  selectSearchState,
  (state) => {
    return state.uploadJobs.entities[id] ?? null
  }
);

export const selectActiveUploadJobs = createSelector(
  selectSearchState,
  (state) => {
    let jobs = uploadJobSelectors.selectAll(state.uploadJobs);

    let count = 0;

    for (let job of jobs) {
      if (!job.done) {
        count++;
      }
    }

    return count;
  }
)

export const selectCurrentJobs = createSelector(
  selectSearchState,
  (state) => {
    let jobs = uploadJobSelectors.selectAll(state.uploadJobs);

    if (!state.showFinishedJobs) {
      jobs = jobs.filter(x => !x.done);
    }

    return jobs;
  }
)

export const selectNextPage = createSelector(
  selectSearchState,
  (state) => {
    return new PageParams(state.pageSize, state.posts.length);
  }
)



export const selectNextItemPage = createSelector(
  selectSearchState,
  (state) => {
    return new PageParams(state.pageSize, state.itemList.length);
  }
)

export const selectNextTagEditSearchPage = createSelector(
  selectSearchState,
  (state) => {
    return new PageParams(state.pageSize, state.tagEditSearchTags.length);
  }
)

export const selectSearchTags = createSelector(
  selectSearchState,
  (state) => {
    return tagSelectors.selectAll(state.searchTags);
  }
);

export {
  selectPageSize,
  selectNextPageLoadingState,
  selectShowSidebar,
  selectCurrentItemLoadingState,
  selectCurrentItem,
  selectSidebarPost,
  selectSidebarPostLoadingState,
  selectViewedPost,
  selectViewedPostLoadingState,
  selectSearchQuery,
  selectViewedPostMode,
  selectItemList,
  selectPostCount,
  selectItemListLoadingState,
  selectTagEditSearchText,
  selectTagEditSearchLoadingState,
  selectTagEditSearchTagCount,
  selectTagEditSearchTags,
  selectTagEditSelectedTagLoadingState,
  selectTagEditSelectedTag
} from './search.reducer'
