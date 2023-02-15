import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Post} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-action-ribbon',
  templateUrl: './action-ribbon.component.html',
  styleUrls: ['./action-ribbon.component.scss']
})
export class ActionRibbonComponent {

  @Input()
  public showPostActions = false;

  @Output()
  public deleteSelected = new EventEmitter<Post>();

  @Output()
  public toggleInfo = new EventEmitter();

  @Output()
  public uploadFiles = new EventEmitter();

  @Output()
  public showUploadProgress = new EventEmitter();

  @Input()
  public activeJobs = 0;

  @Input()
  public selectedPost: Post | null = null;
}
