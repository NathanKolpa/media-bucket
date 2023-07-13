import {ChangeDetectionStrategy, Component, ElementRef, EventEmitter, Input, Output, ViewChild} from '@angular/core';
import {PostSearchQuery, SearchQueryOrder, Tag} from "@core/models";

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-search-bar',
  templateUrl: './search-bar.component.html',
  styleUrls: ['./search-bar.component.scss']
})
export class SearchBarComponent {

  @ViewChild('input')
  public input?: ElementRef<HTMLInputElement>;
  @Output()
  public searchTextChange = new EventEmitter<string | null>();
  @Input()
  public tags: Tag[] = [];
  @Output()
  public queryChange = new EventEmitter<PostSearchQuery>();

  public _query: PostSearchQuery | null = null;

  @Input()
  public set query(query: PostSearchQuery | null) {
    this._query = query;
  }

  addTag(tag: Tag) {
    if (this._query) {
      if (this.input) {
        this.input.nativeElement.value = '';
      }

      this._query = this._query.addTag(tag);
    }
  }

  addText(str: string) {
    if (this._query) {
      if (this.input) {
        this.input.nativeElement.value = '';
      }

      this._query = this._query.addText(str);
    }
  }

  removeIndex(index: number) {
    if (this._query) {
      this._query = this._query.removeItem(index);
    }
  }

  submit() {
    if (this._query) {
      this._query = this._query.randomizeSeed();
      this.queryChange.emit(this._query);
    }
  }

  setOrder(value: SearchQueryOrder) {
    if (this._query) {
      this._query = this._query.setOrder(value);
    }
  }
}
