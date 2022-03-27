import {invoke} from "@tauri-apps/api";

export async function search(kw) {

    let res = await invoke('my_custom_command', {
        number: 0,
        kw: kw,
        isDirOpt: undefined,
        extOpt: undefined,
        parentDirsOpt: undefined,
    });
    return res.file_views;
}