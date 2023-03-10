import {ChangeDetectionStrategy, Component, ElementRef, EventEmitter, Input, Output, ViewChild} from '@angular/core';
import {PostSearchQuery, Tag} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-search-bar',
  templateUrl: './search-bar.component.html',
  styleUrls: ['./search-bar.component.scss']
})
export class SearchBarComponent {

  @ViewChild('input')
  public input?: ElementRef<HTMLInputElement>;

  public _query: PostSearchQuery | null = null;

  @Output()
  public searchTextChange = new EventEmitter<string | null>();

  @Input()
  public tags: Tag[] = [];

  @Input()
  public set query(query: PostSearchQuery | null) {
    this._query = query;
  }

  @Output()
  public queryChange = new EventEmitter<PostSearchQuery>();

  addTag(tag: Tag) {
    if (this._query) {
      if (this.input) {
        this.input.nativeElement.value = '';
      }

      this._query = this._query.addTag(tag);
    }
  }

  removeIndex(index: number) {
    if (this._query) {
      this._query = this._query.removeItem(index);
    }
  }

  submit() {
    if (this._query) {
      this.queryChange.emit(this._query);
    }
  }
}
