type Toc = {
  firstTrackNumber: number;
  lastTrackNumber: number;
  trackData: TrackData[];
}

type TrackData = {
  minutes: number;
  seconds: number;
  frames: number;
}