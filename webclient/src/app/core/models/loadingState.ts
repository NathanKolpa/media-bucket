import {Failure} from "@core/models/failure";

export class LoadingState {

  private constructor(private _isLoading: boolean, private _failure: Failure | null) {
  }

  get isLoading(): boolean {
    return this._isLoading;
  }

  get failure(): Failure | null {
    return this._failure;
  }

  get hasFailure(): boolean {
    return this.failure !== null;
  }

  get isSuccess(): boolean {
    return !this.isLoading && !this.hasFailure
  }

  public static initial(): LoadingState {
    return new LoadingState(false, null);
  }

  public loading(): LoadingState {
    return new LoadingState(true, null)
  }

  public success(): LoadingState {
    return new LoadingState(false, null);
  }

  public fail(failure: Failure): LoadingState {
    return new LoadingState(false, failure);
  }
}
