import { invoke } from "@tauri-apps/api/tauri";
import { createSignal, onCleanup } from "solid-js";

type CdDriveStatus = {
  track_number: number;
  index_number: number;
  absolute_play_time: PlayTime;
  track_relative_play_time: PlayTime;
};

type PlayTime = {
  minutes: number;
  seconds: number;
  frames: number;
}

export function LcdDisplay() {
  const [cdDriveStatus, setCdDriveStatus] = createSignal<CdDriveStatus | null>(null);
  
  let timer: number;

  function timeout() {
    getPlayTime().then(() => {
      console.log("get_play_time");
      timer = setTimeout(() => timeout(), 1000);
    });
  }

  timeout();
  onCleanup(() => clearTimeout(timer));

  async function getPlayTime() {
    setCdDriveStatus(await invoke("get_play_time") as any);
  }

  return <div class="flex flex-row">
    <dl>
      <dt>Track number</dt>
      <dd class="7seg">{cdDriveStatus()?.track_number}</dd>

      <dt>Index number</dt>
      <dd class="7seg">{cdDriveStatus()?.index_number}</dd>

      <dt>Play time</dt>
      <dd class="7seg">{cdDriveStatus()?.index_number === 0 ? '-' : ''}{cdDriveStatus()?.track_relative_play_time.minutes.toString().padStart(2, '0')}:{cdDriveStatus()?.track_relative_play_time.seconds.toString().padStart(2, '0')}</dd>
  </dl>
  </div>
}