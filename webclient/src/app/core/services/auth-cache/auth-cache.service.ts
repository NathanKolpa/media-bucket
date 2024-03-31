import { Injectable } from '@angular/core';
import { Auth } from "@core/models";

const STORAGE_KEY = 'auth';

interface SavedAuth {
  id: number;
  domain: string;
  path: string;
  port: string | null;
  protocol: string;
  shareToken: string;
  lifetime: number;
  createdAt: string
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

    this.saveCookie(auth);
  }

  public remove(auth: Auth) {
    if (auth.privateSession) {
      this.session = this.session.filter(x => x.bucketId != auth.bucketId);
      this.saveSession();
    } else {
      this.storage = this.storage.filter(x => x.bucketId != auth.bucketId);
      this.saveStorage();
    }

    this.removeCookie(auth);
  }

  private loadDriver(driver: Storage, privateSession: boolean): Auth[] {
    let storedStr = driver.getItem(STORAGE_KEY);

    if (storedStr === null) {
      return [];
    }

    let stored: SavedAuth[] = JSON.parse(storedStr);

    return stored
      .map(x => new Auth(x.id, null, x.shareToken, privateSession, x.domain, x.path, x.protocol, x.port, x.lifetime, new Date(x.createdAt), null))
      .filter(x => !x.isExpired());
  }

  private saveSession() {
    this.saveDriver(sessionStorage, this.session);
  }

  private saveStorage() {
    this.saveDriver(localStorage, this.storage);
  }

  private saveDriver(driver: Storage, array: Auth[]) {
    let savedAuth: SavedAuth[] = array.map(auth => ({
      id: auth.bucketId,
      domain: auth.domain,
      path: auth.path,
      port: auth.port,
      protocol: auth.protocol,
      shareToken: auth.shareToken,
      lifetime: auth.lifetime,
      createdAt: auth.createdAt.toISOString()
    }));

    let json = JSON.stringify(savedAuth);

    driver.setItem(STORAGE_KEY, json);
  }

  private saveCookie(auth: Auth) {
    document.cookie = `bucket_${auth.bucketId}=${auth.token}; domain=${auth.domain}; path=${auth.path}; Max-Age=${auth.lifetime}; SameSite=Strict; ${auth.protocol == 'https:' ? 'Secure;' : ''}`
  }

  private removeCookie(auth: Auth) {
    document.cookie = `bucket_${auth.bucketId}=; Max-Age=0; SameSite=Strict`
  }

}
