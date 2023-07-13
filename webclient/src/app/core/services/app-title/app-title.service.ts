import {Injectable} from '@angular/core';
import {Title} from "@angular/platform-browser";

@Injectable({
  providedIn: 'root'
})
export class AppTitleService {

  private stack: string[] = [];
  private readonly initialTitle: string;

  constructor(private title: Title) {
    this.initialTitle = this.title.getTitle();
  }

  public push(name: string): number {
    let index = this.stack.push(name) - 1;
    this.update();

    return index;
  }

  public set(index: number, name: string) {
    this.stack[index] = name;
    this.update();
  }

  public pop() {
    this.stack.pop();
    this.update();
  }

  private update() {
    if (this.stack.length <= 0) {
      this.title.setTitle(this.initialTitle);
      return;
    }

    let titleStr = [...this.stack].reverse().join(' | ');
    this.title.setTitle(titleStr);
  }
}
