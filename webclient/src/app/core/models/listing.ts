import {Media} from "./media";

export interface Listing {
  readonly thumbnail: Media | null;
  readonly title: string | null;
  readonly altTitle: string | null;

  readonly containsImages: boolean;
  readonly containsVideos: boolean;
  readonly containsMovingImages: boolean;
  readonly containsDocument: boolean;
  readonly itemCount: number;
  readonly duration: number | null;
}
