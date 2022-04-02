import React, {useEffect, useState} from 'react';
import {createTheme, ThemeProvider} from "@fluentui/react";
import App from "./App";
import {get_theme} from "./utils";


const default_theme = createTheme({
    palette: {
        themePrimary: '#1a2a3a',
        themeLighterAlt: '#d8e0e7',
        themeLighter: '#b7c5d2',
        themeLight: '#99abbc',
        themeTertiary: '#7d92a7',
        themeSecondary: '#647b91',
        themeDarkAlt: '#4e657b',
        themeDark: '#3a5066',
        themeDarker: '#293d50',
        neutralLighterAlt: '#faf9f8',
        neutralLighter: '#f3f2f1',
        neutralLight: '#edebe9',
        neutralQuaternaryAlt: '#e1dfdd',
        neutralQuaternary: '#d0d0d0',
        neutralTertiaryAlt: '#c8c6c4',
        neutralTertiary: '#a19f9d',
        neutralSecondary: '#605e5c',
        neutralPrimaryAlt: '#3b3a39',
        neutralPrimary: '#323130',
        neutralDark: '#201f1e',
        black: '#000000',
        white: '#ffffff',
    }});
const light_purple  =  createTheme({
    palette: {
        themePrimary: '#562992',
        themeLighterAlt: '#f7f3fb',
        themeLighter: '#ded2ed',
        themeLight: '#c3aede',
        themeTertiary: '#8f6bbd',
        themeSecondary: '#663a9f',
        themeDarkAlt: '#4d2583',
        themeDark: '#411f6e',
        themeDarker: '#301751',
        neutralLighterAlt: '#f2f1f3',
        neutralLighter: '#eeedef',
        neutralLight: '#e4e3e5',
        neutralQuaternaryAlt: '#d5d4d6',
        neutralQuaternary: '#cbcacc',
        neutralTertiaryAlt: '#c3c2c4',
        neutralTertiary: '#a19f9d',
        neutralSecondary: '#605e5c',
        neutralPrimaryAlt: '#3b3a39',
        neutralPrimary: '#323130',
        neutralDark: '#201f1e',
        black: '#000000',
        white: '#f9f8fa',
    }});
const light_blue = createTheme({
    palette: {
        themePrimary: '#3566b9',
        themeLighterAlt: '#f5f8fc',
        themeLighter: '#d8e2f4',
        themeLight: '#b8cbea',
        themeTertiary: '#7b9cd6',
        themeSecondary: '#4975c2',
        themeDarkAlt: '#315ca8',
        themeDark: '#294e8d',
        themeDarker: '#1e3968',
        neutralLighterAlt: '#eef1f3',
        neutralLighter: '#eaedef',
        neutralLight: '#e1e3e5',
        neutralQuaternaryAlt: '#d1d4d6',
        neutralQuaternary: '#c8cacc',
        neutralTertiaryAlt: '#c0c2c4',
        neutralTertiary: '#a19f9d',
        neutralSecondary: '#605e5c',
        neutralPrimaryAlt: '#3b3a39',
        neutralPrimary: '#323130',
        neutralDark: '#201f1e',
        black: '#000000',
        white: '#f7f9fb',
    }});
const themes = [default_theme, light_purple,light_blue];
const Theme = () => {
    let [theme,setTheme] = useState(0);
    useEffect(async ()=>{
        let newVar = await get_theme();
        console.log(newVar)
        setTheme(newVar)
    },[])
    return (
        <div>
            <ThemeProvider theme={themes[theme]}>
                <App setTheme={setTheme} theme={theme}/>
            </ThemeProvider>
        </div>
    );
};

export default Theme;