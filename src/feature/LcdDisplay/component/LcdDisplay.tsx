import { invoke } from "@tauri-apps/api/tauri";
import { createSignal, onCleanup } from "solid-js";

type CdDriveStatus = {
  status: string;
  track_number: number;
  index_number: number;
  absolute_play_time: PlayTime;
  track_relative_play_time: PlayTime;
};

type PlayTime = {
  minutes: number;
  seconds: number;
  frames: number;
};

type PlayTimeMillis = {
  minutes: number;
  seconds: number;
  millis: number;
};

export function LcdDisplay() {
  const [cdDriveStatus, setCdDriveStatus] = createSignal<CdDriveStatus | null>(
    null,
  );

  const [guessedPlayTime, setGuessedPlayTime] =
    createSignal<PlayTimeMillis | null>(null);

  let lastGetPlayTime: Date = new Date(0);

  let timer: number;
  let lastGetTime = 0;
  async function timeout() {
    const status = await invoke<CdDriveStatus>("get_play_time");
    setCdDriveStatus(status);

    const minutes = status.track_relative_play_time.minutes;
    const seconds = status.track_relative_play_time.seconds;
    const frames = status.track_relative_play_time.frames;

    /**
     * frames を milliseconds に変換した値。
     * CD-DA 規格で、1 second は 75 frames で構成されている。
     */
    const millis = (frames * 100) / 75;

    lastGetPlayTime = new Date(minutes * 60 * 1000 + seconds * 1000 + millis);
    lastGetTime = Date.now();

    timer = setTimeout(timeout, 5000);
  }
  timeout();
  onCleanup(() => clearTimeout(timer));

  let animationTimer: number;
  function animate() {
    const currentTime = Date.now();
    const timeDiff = currentTime - lastGetTime;

    const guessedPlayTime = new Date(lastGetPlayTime.getTime() + timeDiff);

    setGuessedPlayTime({
      minutes: guessedPlayTime.getMinutes(),
      seconds: guessedPlayTime.getSeconds(),
      millis: guessedPlayTime.getMilliseconds(),
    });

    animationTimer = requestAnimationFrame(animate);
  }
  animate();
  onCleanup(() => cancelAnimationFrame(animationTimer));

  return (
    <div class="flex justify-center gap-4 w-full py-4 bg-black text-white">
      <p class="font-14seg font-light italic text-3xl">
        {cdDriveStatus()?.status}
      </p>
      <dl>
        <dt class="uppercase">Track number</dt>
        <dd class="font-14seg font-light italic text-3xl">
          {cdDriveStatus()?.track_number}
        </dd>
      </dl>
      <dl>
        <dt class="uppercase">Index number</dt>
        <dd class="font-14seg font-light italic text-xl">
          {cdDriveStatus()?.index_number}
        </dd>
      </dl>
      <dl>
        <dt class="uppercase">Play time</dt>
        <dd class="font-14seg font-light italic text-3xl">
          {`${cdDriveStatus()?.index_number === 0 ? "-" : ""}${
            guessedPlayTime()?.minutes
          }`.padStart(3, "!")}
          :{guessedPlayTime()?.seconds.toString().padStart(2, "0")}.
          <span class="text-xl">
            {guessedPlayTime()?.millis.toString().padStart(3, "0")}
          </span>
        </dd>
      </dl>
    </div>
  );
}
