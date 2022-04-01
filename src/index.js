import * as React from 'react';
import * as ReactDOM from 'react-dom';
import './index.css'
import App from "./App";
import {mergeStyles, ThemeProvider, initializeIcons, createTheme} from '@fluentui/react';
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
        lng: lang,
        fallbackLng: "en",
        interpolation: {
            escapeValue: false
        }
    });
const theme = createTheme({
    palette: {
        themePrimary: '#0078d4',
        themeLighterAlt: '#eff6fc',
        themeLighter: '#deecf9',
        themeLight: '#c7e0f4',
        themeTertiary: '#71afe5',
        themeSecondary: '#2b88d8',
        themeDarkAlt: '#106ebe',
        themeDark: '#005a9e',
        themeDarker: '#004578',
        neutralLighterAlt: '#323130',
        neutralLighter: '#31302f',
        neutralLight: '#2f2e2d',
        neutralQuaternaryAlt: '#2c2b2a',
        neutralQuaternary: '#2a2928',
        neutralTertiaryAlt: '#282726',
        neutralTertiary: '#c8c8c8',
        neutralSecondary: '#d0d0d0',
        neutralPrimaryAlt: '#dadada',
        neutralPrimary: '#ffffff',
        neutralDark: '#f4f4f4',
        black: '#f8f8f8',
        white: '#323130',
    }});
ReactDOM.render(<>
    <ThemeProvider theme={theme}>
        <App/>
    </ThemeProvider>
</>, document.getElementById('root'));


