import {ChangeDetectionStrategy, Component} from '@angular/core';

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-login-tile',
  templateUrl: './login-tile.component.html',
  styleUrls: ['./login-tile.component.scss']
})
export class LoginTileComponent {

}
