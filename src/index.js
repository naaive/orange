import * as React from 'react';
import * as ReactDOM from 'react-dom';
import './index.css'
import App from "./App";
import {mergeStyles, ThemeProvider, initializeIcons} from '@fluentui/react';
import i18n from "i18next";
import { useTranslation, initReactI18next } from "react-i18next";

initializeIcons()
// Inject some global styles
mergeStyles({
    ':global(body,html,#root)': {
        margin: 0,
        padding: 0,
        height: '100vh',
    },
});
const lang = navigator.language || navigator.userLanguage;
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
                    "photo":"Add",
                    "video":"Video",
                    "document":"Document",
                    "folder":"Folder",
                    "name":"Name",
                    "last-modified":"Last Modified",
                    "size":"Size",
                    "path":"Path",

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
                }
            }
        },
        lng: "zh-CN",
        fallbackLng: "en",
        interpolation: {
            escapeValue: false
        }
    });

ReactDOM.render(<>
    <ThemeProvider>
        <App/>
    </ThemeProvider>
</>, document.getElementById('root'));


