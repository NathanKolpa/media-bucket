import {ChangeDetectionStrategy, Component, Inject} from '@angular/core';
import {DIALOG_DATA} from "@angular/cdk/dialog";
import {Post} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-confirm-delete-post-dialog',
  templateUrl: './confirm-delete-post-dialog.component.html',
  styleUrls: ['./confirm-delete-post-dialog.component.scss']
})
export class ConfirmDeletePostDialogComponent {

  constructor(@Inject(DIALOG_DATA) public post: Post) {
  }
}
