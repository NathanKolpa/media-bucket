import {Injectable} from '@angular/core';
import {Auth} from "@core/models";

const STORAGE_KEY = 'auth';

interface SavedAuth {
  id: number;
  token: string | null;
}

@Injectable({
  providedIn: 'root'
})
export class AuthCacheService {

  private session: Auth[] = [];
  private storage: Auth[] = [];

  constructor() {
  }

  public load(): Auth[] {
    this.session = this.loadDriver(sessionStorage, true);
    this.storage = this.loadDriver(localStorage, false);

    return [...this.session, ...this.storage];
  }

  public store(auth: Auth) {
    if (auth.privateSession) {
      this.session.push(auth);
      this.saveSession();
    } else {
      this.storage.push(auth);
      this.saveStorage();
    }
  }

  private loadDriver(driver: Storage, privateSession: boolean): Auth[] {
    let storedStr = driver.getItem(STORAGE_KEY);

    if (storedStr === null) {
      return [];
    }

    let stored: SavedAuth[] = JSON.parse(storedStr);

    return stored.map(x => new Auth(x.id, x.token, privateSession));
  }

  public remove(auth: Auth) {
    if (auth.privateSession) {
      this.session = this.session.filter(x => x.bucketId != auth.bucketId);
      this.saveSession();
    } else {
      this.storage = this.storage.filter(x => x.bucketId != auth.bucketId);
      this.saveStorage();
    }
  }

  private saveSession() {
    this.saveDriver(sessionStorage, this.session);
  }

  private saveStorage() {
    this.saveDriver(localStorage, this.storage);
  }

  private saveDriver(driver: Storage, array: Auth[]) {
    let savedAuth: SavedAuth[] = array.map(auth => ({
      token: auth.token,
      id: auth.bucketId
    }));

    let json = JSON.stringify(savedAuth);

    driver.setItem(STORAGE_KEY, json);
  }
}
