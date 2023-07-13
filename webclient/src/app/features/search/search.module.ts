import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {SearchPageComponent} from './pages/search-page/search-page.component';
import { SearchResultsComponent } from './components/search-results/search-results.component';
import {ScrollingModule} from "@angular/cdk/scrolling";
import { ListingComponent } from './components/listing/listing.component';
import {SearchStoreModule} from "./store";
import {MatCardModule} from "@angular/material/card";
import {MatIconModule} from "@angular/material/icon";
import {PipesModule} from "@shared/pipes/pipes.module";
import {LoadingModule} from "@shared/loading/loading.module";
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatButtonModule} from "@angular/material/button";
import {PostDetailSidebarComponent} from "./components/post-detail-sidebar/post-detail-sidebar.component";
import { ActionRibbonComponent } from './components/action-ribbon/action-ribbon.component';
import {MatToolbarModule} from "@angular/material/toolbar";
import {MatRippleModule} from "@angular/material/core";
import {MatTabsModule} from "@angular/material/tabs";
import {RouterModule, Routes} from "@angular/router";
import { PostDetailDialogComponent } from './containers/post-detail-dialog/post-detail-dialog.component';
import {MatDialogModule} from "@angular/material/dialog";
import {MatMenuModule} from "@angular/material/menu";
import { MediaDisplayComponent } from './components/media-display/media-display.component';
import { PostInfoComponent } from './components/post-info/post-info.component';
import {PdfJsViewerModule} from "ng2-pdfjs-viewer";
import {MatListModule} from "@angular/material/list";
import { UploadDialogComponent } from './containers/upload-dialog/upload-dialog.component';
import {ReactiveFormsModule} from "@angular/forms";
import {MatCheckboxModule} from "@angular/material/checkbox";
import {MatInputModule} from "@angular/material/input";
import {MatRadioModule} from "@angular/material/radio";
import { PostInfoFormFieldsComponent } from './components/post-info-form-fields/post-info-form-fields.component';
import { FileUploadBoxComponent } from './components/file-upload-box/file-upload-box.component';
import { UploadListComponent } from './components/upload-list/upload-list.component';
import { CreatePostFormComponent } from './components/create-post-form/create-post-form.component';
import {MatExpansionModule} from "@angular/material/expansion";
import {CdkDrag, CdkDropList} from "@angular/cdk/drag-drop";
import {MatSnackBarModule} from "@angular/material/snack-bar";
import { TagListComponent } from './components/tag-list/tag-list.component';
import {MatChipsModule} from "@angular/material/chips";
import { TagEditComponent } from './components/tag-edit/tag-edit.component';
import {MatAutocompleteModule} from "@angular/material/autocomplete";
import { UploadProgressDialogComponent } from './containers/upload-progress-dialog/upload-progress-dialog.component';
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {MatBadgeModule} from "@angular/material/badge";
import { ConfirmDeletePostDialogComponent } from './components/confirm-delete-post-dialog/confirm-delete-post-dialog.component';
import { VideoPlayerComponent } from './components/video-player/video-player.component';
import {ConfirmGuard} from "@core/services";
import { SearchBarComponent } from './components/search-bar/search-bar.component';
import { TagComponent } from './components/tag/tag.component';
import { PdfViewerComponent } from './components/pdf-viewer/pdf-viewer.component';
import { ImageViewerComponent } from './components/image-viewer/image-viewer.component';
import {MatTooltipModule} from "@angular/material/tooltip";
import {MatSelectModule} from "@angular/material/select";

const routes: Routes = [
  {
    path: '',
    component: SearchPageComponent,
    canDeactivate: [ConfirmGuard]
  }
]

@NgModule({
  declarations: [
    SearchPageComponent,
    SearchResultsComponent,
    ListingComponent,
    PostDetailSidebarComponent,
    ActionRibbonComponent,
    PostDetailDialogComponent,
    MediaDisplayComponent,
    PostInfoComponent,
    UploadDialogComponent,
    PostInfoFormFieldsComponent,
    FileUploadBoxComponent,
    UploadListComponent,
    CreatePostFormComponent,
    TagListComponent,
    TagEditComponent,
    UploadProgressDialogComponent,
    ConfirmDeletePostDialogComponent,
    VideoPlayerComponent,
    SearchBarComponent,
    TagComponent,
    PdfViewerComponent,
    ImageViewerComponent,
  ],
  exports: [
    SearchPageComponent
  ],
    imports: [
        CommonModule,
        RouterModule.forChild(routes),
        SearchStoreModule,

        PdfJsViewerModule,

        ScrollingModule,
        MatDialogModule,
        MatCardModule,
        MatIconModule,
        PipesModule,
        LoadingModule,
        MatSidenavModule,
        MatButtonModule,
        MatToolbarModule,
        MatRippleModule,
        MatTabsModule,
        MatMenuModule,
        MatListModule,
        ReactiveFormsModule,
        MatCheckboxModule,
        MatInputModule,
        MatRadioModule,
        MatExpansionModule,
        CdkDropList,
        CdkDrag,
        MatSnackBarModule,
        MatChipsModule,
        MatAutocompleteModule,
        MatProgressBarModule,
        MatBadgeModule,
        MatTooltipModule,
        MatSelectModule
    ]
})
export class SearchModule {
}
