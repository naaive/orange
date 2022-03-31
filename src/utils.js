import {invoke} from "@tauri-apps/api";

const fileType2ext = {
    4: "bmp jpg gif png jpeg",
    3: "mp4 mov avi flv f4v mkv",
    2: "doc txt pdf ppt pptx docx xlsx xls",
}
export async function suggest(kw) {
    return await invoke('suggest', {
        kw: kw
    });
}
export async function search(kw, no) {
    let ext = fileType2ext[no];
    let dirOpt = undefined;
    if (no !== undefined) {
        if (no === '1') {
            dirOpt = true;
        }
    }
    return await invoke('search', {
        kw: kw,
        isDirOpt: dirOpt,
        extOpt: ext,
    });
}


export function open_file_location_in_terminal(row) {
    let _ = invoke('open_file_in_terminal', {
        kw: row.abs_path
    });
}

export function open_file_location(row) {
    let _ = invoke('open_file_path', {
        kw: row.abs_path
    });
}