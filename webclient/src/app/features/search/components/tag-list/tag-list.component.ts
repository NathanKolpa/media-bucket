import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from '@angular/core';
import {Tag, TagGroup} from "@core/models";

interface ReverseTagGroup {
  group: TagGroup | null,
  tags: Tag[]
}

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-tag-list',
  templateUrl: './tag-list.component.html',
  styleUrls: ['./tag-list.component.scss']
})
export class TagListComponent {

  tagGroups: ReverseTagGroup[] = [];

  @Input()
  public showDelete = false;

  @Output()
  public removeTag = new EventEmitter<Tag>();

  @Output()
  public clickTag = new EventEmitter<Tag>();

  @Input()
  public set tags(tags: Tag[]) {
    let groups: ReverseTagGroup[] = [];


    for (let tag of tags) {
      let existingTagGroup = groups.find(x => x.group?.id === tag.group?.id);

      if (existingTagGroup !== undefined) {
        existingTagGroup.tags.push(tag);
        continue;
      }

      let newGroup = {
        group: tag.group,
        tags: [tag]
      };

      groups.push(newGroup);
    }

    this.tagGroups = groups;
  }
}
