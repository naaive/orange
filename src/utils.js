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



export function open_file_location_in_terminal(row) {
    invoke('my_custom_command', {
        number: 3,
        kw: row.abs_path
    })
}

export function open_file_location(row) {
    invoke('my_custom_command', {
        number: 1,
        kw: row.abs_path
    })
}