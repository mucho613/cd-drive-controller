import { invoke } from "@tauri-apps/api/tauri";
import { useToc } from "../../../tocContext";
import { Toc } from "../../TocList/types/toc";
import { play } from "../../TocList/component/TocList";
import { Button } from "./Button";

export function ControlPanel() {
  const toc = useToc();

  async function sendCommand(command: string) {
    await invoke("command", { command });
  }

  async function getToc() {
    if (!toc) return;
    const tocData = await invoke<Toc>("get_toc");
    toc[1](tocData);
  }

  if (!toc) return null;
  const tocData = toc[0]();

  return (
    <div class="flex justify-between w-[300px] mx-auto">
      {tocData && <Button label={"Play"} onClick={() => play(tocData, 0)} />}
      <Button label="Stop" onClick={() => sendCommand("stop")} />
      <Button label="Eject" onClick={() => sendCommand("eject")} />
      <Button label="Get TOC" onClick={getToc} />
    </div>
  );
}
