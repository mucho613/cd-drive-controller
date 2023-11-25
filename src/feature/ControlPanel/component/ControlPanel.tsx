import { invoke } from "@tauri-apps/api/tauri";
import { createSignal } from "solid-js";

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

export function ControlPanel() {
  const [commandResult, setCommandResult] = createSignal("");
  const [command, setCommand] = createSignal("");

  const [toc, setToc] = createSignal<Toc | null>(null);

  async function sendCommand() {
    setCommandResult(await invoke("command", { command: command() }));
  }
  
  async function getToc() {
    const toc = await invoke("get_toc") as Toc;
    console.log(toc);
    setToc(toc);
  }

  return <>
    <input
      onChange={(e) => setCommand(e.currentTarget.value)}
    />
    <button onClick={sendCommand}>Send</button>
    <p>{commandResult()}</p>

    <button>Prev track</button>
    <button>Play / Pause</button>
    <button>Stop</button>
    <button>Next track</button>
    <button>Eject</button>
    <button onClick={getToc}>Get TOC</button>
  </>
}