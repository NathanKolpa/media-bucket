import {createAction, props} from "@ngrx/store";
import {
  Failure,
  Media,
  Page,
  Post,
  PostDetail,
  PostItem,
  PostItemDetail,
  PostSearchQuery,
  SearchPost, SearchPostItem,
  SelectedBucket,
  Tag,
  UploadJob
} from "@core/models";

export const showUploadDialog = createAction('[Search] show upload dialog');

export const reset = createAction('[Search] reset');

export const refreshLoaded = createAction('[Search] refresh loaded', props<{ bucket: SelectedBucket }>());
export const refreshLoadedSuccess = createAction('[Search] refresh loaded success', props<{ posts: SearchPost[] }>());
export const refreshLoadedFailure = createAction('[Search] refresh loaded failure', props<{ failure: Failure }>());

export const loadNext = createAction('[Search] load next', props<{ bucket: SelectedBucket }>());
export const loadNextSuccess = createAction('[Search] load next success', props<{ page: Page, posts: SearchPost[] }>());
export const loadNextFailure = createAction('[Search] load next failure', props<{ failure: Failure }>());

export const toggleInfo = createAction('[Search] toggle info');
export const closeInfo = createAction('[Search] close info');

export const showPost = createAction('[Search] show post', props<{ bucket: SelectedBucket, postId: number, showPopup?: boolean }>());
export const showPostSuccess = createAction('[Search] show post success', props<{ post: PostDetail }>());
export const showPostFailure = createAction('[Search] show post failure', props<{ failure: Failure }>());

export const showPostSidebar = createAction('[Search] show post sidebar', props<{ bucket: SelectedBucket, postId: number }>());
export const showPostSidebarSuccess = createAction('[Search] show post sidebar success', props<{ post: PostDetail }>());
export const showPostSidebarFailure = createAction('[Search] show post sidebar failure', props<{ failure: Failure }>());

export const loadPostItem = createAction('[Search] load post item', props<{ bucket: SelectedBucket, postId: number, position: number }>());
export const loadPostItemSuccess = createAction('[Search] load post item success', props<{ item: PostItemDetail }>());
export const loadPostItemFailure = createAction('[Search] load post item failure', props<{ failure: Failure }>());

export const uploadProgress = createAction('[Search] upload progress', props<{ jobId: string, index: number, uploadedBytes: number }>());
export const uploadDone = createAction('[Search] upload done', props<{ jobId: string, index: number, content: Media, thumbnail: Media }>());
export const uploadFailure = createAction('[Search] upload failure', props<{ jobId: string, index: number, failure: Failure }>());
export const startUploadJob = createAction('[Search] start upload job', props<{ bucket: SelectedBucket, job: UploadJob }>());
export const uploadJobFailure = createAction('[Search] upload job failure', props<{ jobId: string, failure: Failure }>());
export const uploadJobPostCreatedSuccess = createAction('[Search] upload job post created success', props<{ bucket: SelectedBucket, jobId: string, posts: Post[], batchId: number }>());
export const swapUpload = createAction('[Search] swap upload', props<{ jobId: string, aIndex: number, bIndex: number }>());
export const deleteUploads = createAction('[Search] delete uploads', props<{ jobId: string, indexes: number[] }>());

export const showUploadProgress = createAction('[Search] show upload progress');

export const requestPostDelete = createAction('[Search] request post delete', props<{ bucket: SelectedBucket, post: Post }>());
export const deletePost = createAction('[Search] delete post', props<{ bucket: SelectedBucket, postId: number }>());
export const deletePostSuccess = createAction('[Search] delete post success', props<{ postId: number }>());
export const deletePostFailure = createAction('[Search] delete post failure', props<{ failure: Failure }>());

export const searchQueryChange = createAction('[Search] search query change', props<{ bucket: SelectedBucket, query: PostSearchQuery }>());
export const addTagToSearchQuery = createAction('[Search] add tag to search query', props<{ bucket: SelectedBucket, tag: Tag }>());

export const searchTextChange = createAction('[Search] search text change', props<{ bucket: SelectedBucket, query: string | null }>());
export const searchTagSuccess = createAction('[Search] search tags success', props<{ tags: Tag[] }>());
export const searchTagFailure = createAction('[Search] search tags failure', props<{ failure: Failure }>());

export const togglePostDetailViewMode = createAction('[Search] toggle post detail view mode');

export const loadNextPostItemList = createAction('[Search] load next post item', props<{ bucket: SelectedBucket, postId: number }>());
export const loadNextPostItemListSuccess = createAction('[Search] load next post item success', props<{ items: SearchPostItem[] }>());
export const loadNextPostItemListFailure = createAction('[Search] load next post item failure', props<{ failure: Failure }>());

export const updatePost = createAction('[Search] update post', props<{ bucket: SelectedBucket, postId: number, title: string | null, description: string | null, source: string | null, tags: Tag[] }>());
export const updatePostSuccess = createAction('[Search] update post success', props<{ post: Post, tags: Tag[] }>());
export const updatePostFailure = createAction('[Search] update post failure', props<{ failure: Failure }>());
