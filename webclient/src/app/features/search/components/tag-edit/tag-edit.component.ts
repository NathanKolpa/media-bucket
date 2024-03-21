import { Component, EventEmitter, Input, OnDestroy, Output } from '@angular/core';
import { PageParams, SelectedBucket, Tag } from "@core/models";
import { FormControl } from "@angular/forms";
import { auditTime, combineLatest, debounceTime, map, startWith, Subject, Subscription, switchMap } from "rxjs";
import { ApiService } from "@core/services";
import { MatSnackBar } from "@angular/material/snack-bar";

// TODO: this components should make api requests through ngrx
// for now its alright because it doesn't effect other parts of the system
// however this will need to be fixed when the tag edit dialog gets implemented
// idea make from some components a container so we dont have to hook up so many events

@Component({
  selector: 'app-tag-edit',
  templateUrl: './tag-edit.component.html',
  styleUrls: ['./tag-edit.component.scss']
})
export class TagEditComponent implements OnDestroy {
  @Output()
  public tagClick = new EventEmitter<Tag>();

  public searchTags: Tag[] = [];
  @Input()
  public tags: Tag[] = [];
  @Output()
  public tagsChange = new EventEmitter<Tag[]>();
  public searchField = new FormControl();
  private bucket$: Subject<SelectedBucket> = new Subject<SelectedBucket>();
  private tagsSubscription: Subscription;

  constructor(private api: ApiService, private snackBar: MatSnackBar) {
    let query = this.searchField.valueChanges.pipe(
      startWith(''),
      auditTime(250)
    );

    this.tagsSubscription = combineLatest([query, this.bucket$]).pipe(
      switchMap(([query, bucket]) => this.api.searchTags(bucket.auth, new PageParams(25, 0), query ?? '').pipe(
        map(x => x.tags)
      )),
    ).subscribe(tags => this.searchTags = tags)
  }

  private _bucket: SelectedBucket | null = null;

  @Input()
  public set bucket(value: SelectedBucket | null) {
    if (value) {
      this.bucket$.next(value);
      this._bucket = value;
    }
  }

  ngOnDestroy(): void {
    this.tagsSubscription.unsubscribe();
  }

  addTag(tag: Tag) {
    if (this.tagSelected(tag)) {
      return;
    }

    this.tagsChange.emit([...this.tags, tag]);
  }

  removeTag(tag: Tag) {
    this.tagsChange.emit(this.tags.filter(x => x.id != tag.id))
  }

  tagSelected(tag: Tag): boolean {
    return this.tags.find(x => x.id == tag.id) !== undefined;
  }

  showCreateTag(): boolean {
    return this.searchField.value?.length > 0 && this.searchTags.find(x => x.name == this.searchField.value) === undefined
  }

  createTag() {
    if (this._bucket == null) {
      return;
    }

    let bucket = this._bucket;

    this.api.createTag(bucket.auth, this.searchField.value + '', null).subscribe(tag => {
      this.addTag(tag);

      let snackBar = this.snackBar.open(`Successfully created tag "${tag.name}"`, 'Undo', {
        duration: 3000
      });

      snackBar.onAction().subscribe(() => {
        this.api.removeTag(bucket.auth, tag.id).subscribe(() => {
          this.removeTag(tag)
          this.searchTags = this.searchTags.filter(x => x.id != tag.id);
        });
      })
    })
  }
}
