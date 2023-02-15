import {ChangeDetectionStrategy, Component, ElementRef, EventEmitter, Output, ViewChild} from '@angular/core';

@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  selector: 'app-file-upload-box',
  templateUrl: './file-upload-box.component.html',
  styleUrls: ['./file-upload-box.component.scss']
})
export class FileUploadBoxComponent {

  @Output()
  filesAdded = new EventEmitter<File[]>();

  @ViewChild('input')
  input!: ElementRef<HTMLInputElement>;

  highlighted = false;

  highlight() {
    this.highlighted = true;
  }

  unHighlight() {
    this.highlighted = false;
  }

  dragLeave(event: any) {
    this.unHighlight();
  }

  dragEnter(event: any) {
    this.highlight();
  }

  dragOver(event: any) {
    event.preventDefault();
    this.highlight();
  }


  drop(event: any) {
    event.preventDefault();
    this.unHighlight();
    this.emitFileList(event.dataTransfer.files);
  }

  emitFileList(list: FileList) {
    let files = [];

    for (let i = 0; i < list.length; i++) {
      let file: File = list.item(i)!;
      files.push(file);
    }
    if (files.length > 0) {
      this.filesAdded.emit(files);
    }
  }

  fileSelectChange() {
    this.emitFileList(this.input.nativeElement.files!);
  }

  selectFiles() {
    this.input.nativeElement.click();
  }
}
