import { createFeature, createReducer, on } from "@ngrx/store";
import {
  LoadingState,
  PostDetail,
  PostItemDetail,
  SearchPost,
  SearchPostItem,
  Tag,
  TagDetail,
  UploadJob
} from "@core/models";
import { createEntityAdapter, EntityState } from "@ngrx/entity";
import * as searchActions from "./search.actions";
import { PostSearchQuery } from "@core/models/searchQuery";

interface State {
  searchText: string | null,
  searchQuery: PostSearchQuery,
  searchTags: EntityState<Tag>,
  nextPageLoadingState: LoadingState,
  pageSize: number,
  posts: SearchPost[], // use an array instead of entity state because order is important
  postCount: null | number,

  showSidebar: boolean,
  sidebarPostLoadingState: LoadingState,
  sidebarPost: PostDetail | null,

  viewOffset: number | null,
  viewedPost: PostDetail | null,
  viewedPostLoadingState: LoadingState,
  currentItemLoadingState: LoadingState,
  currentItem: PostItemDetail | null,
  nextItemPreview: SearchPostItem | null,
  prevItemPreview: SearchPostItem | null,
  itemList: SearchPostItem[],
  itemListLoadingState: LoadingState,
  viewedPostMode: 'list' | 'preview',

  showFinishedJobs: boolean,
  uploadJobs: EntityState<UploadJob>,

  tagEditSearchText: string | null,
  tagEditSearchTags: Tag[]
  tagEditSearchLoadingState: LoadingState
  tagEditSearchTagCount: number | null,
  tagEditSelectedTag: TagDetail | null,
  tagEditSelectedTagLoadingState: LoadingState
}

const uploadJobAdapter = createEntityAdapter<UploadJob>();
const tagAdapter = createEntityAdapter<Tag>();

const initialState: State = {
  searchText: null,
  searchQuery: PostSearchQuery.empty(),
  searchTags: tagAdapter.getInitialState(),
  nextPageLoadingState: LoadingState.initial(),
  pageSize: 50,
  posts: [],
  postCount: null,

  viewOffset: null,
  showSidebar: false,
  sidebarPostLoadingState: LoadingState.initial(),
  currentItemLoadingState: LoadingState.initial(),
  sidebarPost: null,
  currentItem: null,
  nextItemPreview: null,
  prevItemPreview: null,
  itemList: [],
  viewedPost: null,
  viewedPostLoadingState: LoadingState.initial(),
  itemListLoadingState: LoadingState.initial(),
  viewedPostMode: 'preview',
  showFinishedJobs: false,
  uploadJobs: uploadJobAdapter.getInitialState(),

  tagEditSearchText: null,
  tagEditSearchTags: [],
  tagEditSearchLoadingState: LoadingState.initial(),
  tagEditSearchTagCount: null,
  tagEditSelectedTag: null,
  tagEditSelectedTagLoadingState: LoadingState.initial()
};

const feature = createFeature({
  name: 'search',
  reducer: createReducer(
    initialState,

    on(searchActions.loadNext, (state) => ({
      ...state,
      nextPageLoadingState: state.nextPageLoadingState.loading()
    })),
    on(searchActions.loadNextSuccess, (state, { posts, page }) => ({
      ...state,
      nextPageLoadingState: state.nextPageLoadingState.success(),
      posts: [...state.posts, ...posts],
      postCount: page.totalRows
    })),
    on(searchActions.loadNextFailure, (state, { failure }) => ({
      ...state,
      nextPageLoadingState: state.nextPageLoadingState.fail(failure)
    })),
    on(searchActions.refreshLoadedSuccess, (state, { posts }) => ({
      ...state,
      posts,
    })),

    on(searchActions.toggleInfo, (state) => ({
      ...state,
      showSidebar: !state.showSidebar,
      sidebarPost: !state.showSidebar ? state.sidebarPost : null
    })),
    on(searchActions.closeInfo, (state) => ({
      ...state,
      showSidebar: false
    })),

    on(searchActions.showPost, (state, { postId, offset }) => {
      if (postId == state.viewedPost?.id) {
        return state;
      }

      return ({
        ...state,
        viewedPostLoadingState: state.viewedPostLoadingState.loading(),
        viewOffset: offset,
        currentItem: null,
        viewedPost: null,
        nextItemPreview: null,
        prevItemPreview: null,
        itemList: [],
      });
    }),
    on(searchActions.showPostSuccess, (state, { post }) => ({
      ...state,
      viewedPostLoadingState: state.viewedPostLoadingState.success(),
      viewedPost: post,
      viewedPostMode: post.itemCount == 1 ? 'preview' : state.viewedPostMode
    })),
    on(searchActions.showPostFailure, (state, { failure }) => ({
      ...state,
      viewedPostLoadingState: state.viewedPostLoadingState.fail(failure)
    })),

    on(searchActions.showPostSidebar, (state) => ({
      ...state,
      sidebarPostLoadingState: state.sidebarPostLoadingState.loading(),
      showSidebar: true,
    })),
    on(searchActions.showPostSidebarSuccess, (state, { post }) => ({
      ...state,
      sidebarPostLoadingState: state.sidebarPostLoadingState.success(),
      sidebarPost: post
    })),
    on(searchActions.showPostSidebarFailure, (state, { failure }) => ({
      ...state,
      sidebarPostLoadingState: state.sidebarPostLoadingState.fail(failure)
    })),

    on(searchActions.loadPostItem, (state) => ({
      ...state,
      currentItemLoadingState: state.currentItemLoadingState.loading()
    })),
    on(searchActions.loadPostItemSuccess, (state, { item }) => ({
      ...state,
      currentItemLoadingState: state.currentItemLoadingState.success(),
      currentItem: item
    })),
    on(searchActions.loadPostItemFailure, (state, { failure }) => ({
      ...state,
      currentItemLoadingState: state.currentItemLoadingState.fail(failure)
    })),

    on(searchActions.loadNeighbourItemsSuccess, (state, { items, startPosition, position, total }) => {
      if (items.length == 0) {
        return state;
      }

      let itemList = [...state.itemList];

      for (let i = 0; i < items.length; i++) {
        let item = items[i];
        itemList[item.position] = item;
      }

      let prevItemPreview = null;
      let nextItemPreview = null;


      if (startPosition == position - 1) {
        prevItemPreview = items[0];
      }

      if (position + 1 < total) {
        nextItemPreview = items[items.length - 1]
      }

      return {
        ...state,
        itemList,
        prevItemPreview,
        nextItemPreview
      }
    }),

    on(searchActions.reset, () => initialState),

    on(searchActions.startUploadJob, (state, { job }) => ({
      ...state,
      uploadJobs: uploadJobAdapter.addOne(job, state.uploadJobs)
    })),
    on(searchActions.uploadJobFailure, (state, { jobId, failure }) => {
      let job = state.uploadJobs.entities[jobId]?.error(failure)

      if (!job) {
        return state;
      }

      return {
        ...state,
        uploadJobs: uploadJobAdapter.setOne(job, state.uploadJobs)
      };
    }),
    on(searchActions.uploadJobPostCreatedSuccess, (state, { jobId }) => {
      let job = state.uploadJobs.entities[jobId]?.postCreated()

      if (!job) {
        return state;
      }

      return {
        ...state,
        uploadJobs: uploadJobAdapter.setOne(job, state.uploadJobs)
      };
    }),

    on(searchActions.uploadProgress, (state, { jobId, index, uploadedBytes }) => {
      let job = state.uploadJobs.entities[jobId]?.mapUpload(index, (u) => u.setProgress(uploadedBytes))

      if (!job) {
        return state;
      }

      return {
        ...state,
        uploadJobs: uploadJobAdapter.setOne(job, state.uploadJobs)
      };
    }),
    on(searchActions.uploadFailure, (state, { jobId, index, failure }) => {
      let job = state.uploadJobs.entities[jobId]?.mapUpload(index, (u) => u.error(failure))

      if (!job) {
        return state;
      }

      return {
        ...state,
        uploadJobs: uploadJobAdapter.setOne(job, state.uploadJobs)
      };
    }),
    on(searchActions.uploadDone, (state, { jobId, index, content, thumbnail }) => {
      let job = state.uploadJobs.entities[jobId]?.mapUpload(index, (u) => u.done(content, thumbnail));

      if (!job) {
        return state;
      }

      return {
        ...state,
        uploadJobs: uploadJobAdapter.setOne(job, state.uploadJobs)
      };
    }),
    on(searchActions.swapUpload, (state, { jobId, aIndex, bIndex }) => {
      let job = state.uploadJobs.entities[jobId]?.moveUploadToIndex(aIndex, bIndex);

      if (!job) {
        return state;
      }

      return {
        ...state,
        uploadJobs: uploadJobAdapter.setOne(job, state.uploadJobs)
      };
    }),
    on(searchActions.deleteUploads, (state, { jobId, indexes }) => {
      let job = state.uploadJobs.entities[jobId]?.deleteUploads(indexes)

      if (!job) {
        return state;
      }

      return {
        ...state,
        uploadJobs: uploadJobAdapter.setOne(job, state.uploadJobs)
      };
    }),

    on(searchActions.deletePostSuccess, (state, { postId }) => {
      let posts = state.posts.filter(x => x.id !== postId);
      let showSidebar = state.showSidebar;
      let sidebarPost = state.sidebarPost;

      if (state.sidebarPost?.id == postId) {
        showSidebar = false;
        sidebarPost = null;
      }

      return {
        ...state,
        posts,
        showSidebar,
        sidebarPost
      };
    }),

    on(searchActions.searchTagSuccess, (state, { tags }) => ({
      ...state,
      searchTags: tagAdapter.setAll(tags, state.searchTags)
    })),
    on(searchActions.searchQueryChange, (state, { query }) => ({
      ...state,
      searchQuery: query,
      posts: []
    })),
    on(searchActions.addTagToSearchQuery, (state, { tag }) => ({
      ...state,
      searchQuery: state.searchQuery.addTag(tag),
      posts: []
    })),
    on(searchActions.viewTagLinkedPosts, (state, { tag }) => ({
      ...state,
      searchQuery: state.searchQuery.setTag(tag),
      posts: []
    })),


    on(searchActions.togglePostDetailViewMode, (state) => ({
      ...state,
      viewedPostMode: state.viewedPostMode == 'list' ? 'preview' : 'list',
    })),


    on(searchActions.loadNextPostItemList, (state) => ({
      ...state,
      itemListLoadingState: state.itemListLoadingState.loading()
    })),
    on(searchActions.loadNextPostItemListSuccess, (state, { items }) => {
      let itemList = [...state.itemList];

      for (let i = 0; i < items.length; i++) {
        let item = items[i];
        itemList[item.position] = item;
      }

      return ({
        ...state,
        itemListLoadingState: state.itemListLoadingState.success(),
        itemList
      });
    }),
    on(searchActions.loadNextPostItemListFailure, (state, { failure }) => ({
      ...state,
      itemListLoadingState: state.itemListLoadingState.fail(failure)
    })),

    on(searchActions.updatePostSuccess, (state, { post, detail }) => ({
      ...state,
      sidebarPost: state.sidebarPost?.id == post.id ?
        detail : state.sidebarPost,
      posts: state.posts.map(searchPost => {
        if (searchPost.id != post.id) {
          return searchPost
        }

        return new SearchPost(searchPost.id, post.source, post.title, post.description, post.createdAt, searchPost.itemCount, searchPost.containsDocument, searchPost.containsImages, searchPost.containsVideos, searchPost.containsMovingImages, searchPost.duration, searchPost.thumbnail, searchPost.filename)
      })
    })),

    on(searchActions.loadTagEditNextSearchTags, (state) => ({
      ...state,
      tagEditSearchLoadingState: state.tagEditSearchLoadingState.loading()
    })),

    on(searchActions.loadTagEditNextSearchTagsSuccess, (state, { tags, page }) => ({
      ...state,
      tagEditSearchLoadingState: state.tagEditSearchLoadingState.success(),
      tagEditSearchTags: [...state.tagEditSearchTags, ...tags],
      tagEditSearchTagCount: page.totalRows
    })),

    on(searchActions.loadTagEditNextSearchTagsFailure, (state, { failure }) => ({
      ...state,
      tagEditSearchLoadingState: state.tagEditSearchLoadingState.fail(failure)
    })),

    on(searchActions.tagEditSearchQueryChange, (state, { query }) => ({
      ...state,
      tagEditSearchTags: [],
      tagEditSearchText: query,
      tagEditSearchLoadingState: state.tagEditSearchLoadingState.loading(),
      tagEditSearchTagCount: null
    })),

    on(searchActions.tagEditClearSelected, (state) => ({
      ...state,
      tagEditSelectedTag: null
    })),

    on(searchActions.tagEditSelectTag, (state) => ({
      ...state,
      tagEditSelectedTagLoadingState: state.tagEditSelectedTagLoadingState.loading()
    })),

    on(searchActions.tagEditSelectTagSuccess, (state, { tag }) => ({
      ...state,
      tagEditSelectedTagLoadingState: state.tagEditSelectedTagLoadingState.success(),
      tagEditSelectedTag: tag
    })),

    on(searchActions.tagEditSelectTagFailure, (state, { failure }) => ({
      ...state,
      tagEditSelectedTagLoadingState: state.tagEditSelectedTagLoadingState.fail(failure),
    })),
  )
});

export const {
  name,
  reducer,
  selectSearchState,
  selectPageSize,
  selectNextPageLoadingState,
  selectCurrentItemLoadingState,
  selectCurrentItem,
  selectShowSidebar,
  selectSidebarPostLoadingState,
  selectViewedPostLoadingState,
  selectViewedPost,
  selectSidebarPost,
  selectItemList,
  selectViewedPostMode,
  selectSearchQuery,
  selectPostCount,
  selectItemListLoadingState,
  selectTagEditSearchText,
  selectTagEditSearchLoadingState,
  selectTagEditSearchTagCount,
  selectTagEditSearchTags,
  selectTagEditSelectedTagLoadingState,
  selectTagEditSelectedTag,
  selectNextItemPreview,
  selectPrevItemPreview,
  selectViewOffset
} = feature;

export const uploadJobSelectors = uploadJobAdapter.getSelectors();
export const tagSelectors = tagAdapter.getSelectors();
