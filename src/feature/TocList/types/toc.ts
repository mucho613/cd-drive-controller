export type Toc = {
  firstTrackNumber: number;
  lastTrackNumber: number;
  trackData: TrackData[];
};

export type TrackData = {
  minutes: number;
  seconds: number;
  frames: number;
};
