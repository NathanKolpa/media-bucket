import {ChangeDetectionStrategy, Component} from '@angular/core';
import {environment} from "@src/environments/environment";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-login-tile',
  templateUrl: './login-tile.component.html',
  styleUrls: ['./login-tile.component.scss']
})
export class LoginTileComponent {
  public apiDocsUrl = environment.api + '/docs/ui'
}
