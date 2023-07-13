import {Component, OnDestroy, OnInit} from '@angular/core';
import {authActions} from '@core/store/auth';
import {Store} from "@ngrx/store";
import {AppTitleService} from "@core/services";
import {environment} from "@src/environments/environment";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit, OnDestroy {
  constructor(private store: Store, private title: AppTitleService) {
  }

  ngOnInit(): void {
    this.store.dispatch(authActions.initialize());

    this.title.push(environment.title)
  }

  ngOnDestroy(): void {
    this.title.pop();
  }
}
