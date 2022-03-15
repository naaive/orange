<div align="center">
<img height=150 src="https://github.com/naaive/orange/blob/master/src-tauri/icons/icon.png" />
</div>
<p align="center">
<span>English</span>
<span> | </span>
<a href="README_cn.md">中文</a>
</p>
<p align="center"><span>A cross-platform desktop application for searching local files.</span></p>



<div align="center">

[![Download Counts](https://img.shields.io/github/downloads/naaive/orange/total?style=flat)](https://github.com/naaive/orange/releases)
[![Stars Count](https://img.shields.io/github/stars/naaive/orange?style=flat)](https://github.com/naaive/orange/stargazers) [![Forks Count](https://img.shields.io/github/forks/naaive/orange.svg?style=flat)](https://github.com/naaive/orange/network/members)
[![LICENSE](https://img.shields.io/badge/license-gpl-green?style=flat)](https://github.com/naaive/orange/blob/master/LICENSE)

[![Windows Support](https://img.shields.io/badge/Windows-0078D6?style=flat&logo=windows&logoColor=white)](https://github.com/naaive/orange/releases)
[![Windows Support](https://img.shields.io/badge/MACOS-adb8c5?style=flat&logo=macos&logoColor=white)](https://github.com/naaive/orange/releases)
[![Linux Support](https://img.shields.io/badge/linux-1793D1?style=flat&logo=linux&logoColor=white)](https://github.com/naaive/orange/releases)
</div>

## What is Orange?

![Demo](screenshot/orange_0.0.5.gif)

Orange is a **file search** desktop application. 

## ✨Features

- Fast search response
- Low memory and low cpu usage
- Easy to use, comes with tokenization and auto completion
- Monitor file changes in real time
- Lightweight installation package
- Simple and elegant UI

## Build 
- Setup Tauri dev environment (https://tauri.studio/docs/getting-started/setting-up-macos)
- Run `yarn`
- Run `yarn build`
- Run `yarn tauri-build`


## Download

Go to [release page](https://github.com/naaive/orange/releases).

## Notice

If you are Macos user, after install, please run `xattr -cr /Applications/Orange.app` to fix `“Orange” is damaged and can’t be opened. You should move it to the Trash.`

## Architecture
![arch](doc/img.png)


## Thanks
- Tauri https://tauri.studio
- Notify https://github.com/notify-rs/notify
- React https://github.com/facebook/react
- Tantivy https://github.com/quickwit-oss/tantivy
- Kv https://github.com/zshipko/rust-kv




## LICENSE

[GPL](https://github.com/naaive/orange/blob/master/LICENSE)



