import { invoke } from "@tauri-apps/api/tauri";
import { useToc } from "../../../tocContext";
import { Toc } from "../types/toc";

export async function play(toc: Toc, trackNumber: number) {
  const track = toc.track_data[trackNumber];
  if(!track) return;

  await invoke("play", { playTime: track });
}

export function TocList() {
  const toc = useToc();

  if(!toc) return null;

  const tocData = toc[0]();

  if(!tocData) return null;

  return (
    <section class="m-4">
      <h2 class="text-xl">TOC</h2>
      <ol class="list-decimal ml-6">
        {tocData.track_data.map((track, index) => (
          <li onClick={() => play(tocData, index)}>
            {track.minutes.toString().padStart(2, " ")}:
            {track.seconds.toString().padStart(2, "0")}
          </li>
        ))}
      </ol>
    </section>
  );
}
