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
};

export function LcdDisplay() {
    const [cdDriveStatus, setCdDriveStatus] =
        createSignal<CdDriveStatus | null>(null);

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
        setCdDriveStatus((await invoke("get_play_time")) as CdDriveStatus);
    }

    return (
        <div class="flex justify-center gap-4 w-full py-4 bg-black text-white">
            <dl>
                <dt class="uppercase">Track number</dt>
                <dd class="font-14seg italic text-3xl">
                    {cdDriveStatus()?.track_number}
                </dd>
            </dl>
            <dl>
                <dt class="uppercase">Index number</dt>
                <dd class="font-14seg italic text-3xl">
                    {cdDriveStatus()?.index_number}
                </dd>
            </dl>
            <dl>
                <dt class="uppercase">Play time</dt>
                <dd class="font-14seg italic text-3xl">
                    {`${cdDriveStatus()?.index_number === 0 ? "-" : ""}${
                        cdDriveStatus()?.track_relative_play_time.minutes
                    }`.padStart(3, "!")}
                    :
                    {cdDriveStatus()
                        ?.track_relative_play_time.seconds.toString()
                        .padStart(2, "0")}
                </dd>
            </dl>
        </div>
    );
}
