import {invoke} from "@tauri-apps/api";
import i18next from "i18next";

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

export async function get_lang() {
    return await invoke('get_lang', {});
}

export function change_lang(lang) {
    let _ = invoke('change_lang', {
        lang: lang
    });

}

export async function walk_metrics() {
    return await invoke('walk_metrics', {});

}

export async function get_theme() {
    return await invoke('get_theme', {});
}

export function change_theme(theme) {
    let _ = invoke('change_theme', {
        theme: theme
    });
}

export async function add_exclude_path(path) {

    return await invoke('add_exclude_path', {
        path: path
    });
}

export async function remove_exclude_path(path) {
    return await invoke('remove_exclude_path', {
        path: path
    });
}

export function upgrade() {
    let _ = invoke('upgrade', {});
}

export function reindex() {
    let _ = invoke('reindex', {});
}

export async function get_exclude_paths() {
    return await invoke('get_exclude_paths', {});
}