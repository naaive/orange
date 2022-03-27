import {invoke} from "@tauri-apps/api";

const fileType2ext = {
    4: "bmp jpg gif png jpeg",
    3: "mp4 mov avi flv f4v mkv",
    2: "doc txt pdf ppt pptx docx xlsx xls",
}

export async function search(kw, no) {

    let ext = fileType2ext[no];
    let dirOpt = undefined;
    if (no !== undefined) {
        if (no === 1) {
            dirOpt = true;
        }
    }
    console.log(dirOpt);
    let res = await invoke('my_custom_command', {
        number: 0,
        kw: kw,
        isDirOpt: dirOpt,
        extOpt: ext,
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