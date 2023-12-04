export type Toc = {
  first_track_number: number;
  last_track_number: number;
  track_data: TrackData[];
};

export type TrackData = {
  minutes: number;
  seconds: number;
  frames: number;
};
