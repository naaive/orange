import * as React from 'react';
import * as ReactDOM from 'react-dom';
import './index.css'
import App from "./App";
import {createTheme, initializeIcons, mergeStyles, ThemeProvider} from '@fluentui/react';
import i18n from "i18next";
import {initReactI18next} from "react-i18next";
import {change_lang, get_lang} from "./utils";
import i18next from "i18next";
import Theme from "./Theme";


initializeIcons()
// Inject some global styles
mergeStyles({
    ':global(body,html,#root)': {
        margin: 0,
        padding: 0,
        height: '100vh',
    },
});
let lang= navigator.language || navigator.userLanguage;
let _ = i18n
    .use(initReactI18next)
    .init({
        resources: {
            "en": {
                translation: {
                    "setting-header": "Setting",
                    "exclude-path-label":"Exclude Path",
                    "add":"Add",
                    "all":"All",
                    "photo":"Photo",
                    "video":"Video",
                    "document":"Document",
                    "folder":"Folder",
                    "name":"Name",
                    "last-modified":"Last Modified",
                    "size":"Size",
                    "path":"Path",
                    "lang":"Language",
                    "theme":"Theme",
                    "theme-default":"Default",
                    "theme-light-purple":"Light Purple",
                    "theme-light-blue":"Light Blue",
                    "reindex":"Reindex",
                    "reindex-dialog":"Do you want to Reindex? It will take effect on next reboot!",
                    "remove":"Remove",
                    "confirm":"Confirm",
                    "cancel":"Cancel",
                    "add_exclude_path_err":"Invalid path",
                    "upgrade":"Upgrade",
                    "version":"Version V"

                }
            },
            "zh-CN":{
                translation: {
                    "setting-header": "设置",
                    "exclude-path-label":"排除路径",
                    "add":"添加",
                    "all":"所有",
                    "photo":"图片",
                    "video":"视频",
                    "document":"文档",
                    "folder":"文件夹",
                    "name":"名称",
                    "last-modified":"上次修改",
                    "size":"大小",
                    "path":"路径",
                    "lang":"语言",
                    "theme":"主题",
                    "theme-default":"默认",
                    "theme-light-purple":"浅紫色",
                    "theme-light-blue":"浅蓝色",
                    "reindex":"重索引",
                    "reindex-dialog":"您确认要重新索引吗？它将在下一次重启时生效！",
                    "remove":"删除",
                    "confirm":"确认",
                    "cancel":"取消",
                    "add_exclude_path_err":"非法路径",
                    "upgrade":"升级",
                    "version":"版本 V"
                }
            }
        },
        lng: lang,
        fallbackLng: "en",
        interpolation: {
            escapeValue: false
        }
    });






ReactDOM.render(<>
    <Theme/>
</>, document.getElementById('root'));



