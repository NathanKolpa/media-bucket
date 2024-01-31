import { TestBed } from '@angular/core/testing';

import { ReloginInterceptor } from './relogin.interceptor';

describe('ReloginInterceptor', () => {
  beforeEach(() => TestBed.configureTestingModule({
    providers: [
      ReloginInterceptor
      ]
  }));

  it('should be created', () => {
    const interceptor: ReloginInterceptor = TestBed.inject(ReloginInterceptor);
    expect(interceptor).toBeTruthy();
  });
});
