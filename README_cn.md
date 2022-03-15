<div align="center">
<img height=150 src="https://github.com/naaive/orange/blob/master/src-tauri/icons/icon.png" />
</div>
<p align="center">
<a href="README.md">English</a>
<span> | </span>
<span >中文</span>
</p>
<p align="center"><span>跨平台的文件搜索引擎</span></p>



<div align="center">

[![Download Counts](https://img.shields.io/github/downloads/naaive/orange/total?style=flat)](https://github.com/naaive/orange/releases)
[![Stars Count](https://img.shields.io/github/stars/naaive/orange?style=flat)](https://github.com/naaive/orange/stargazers) [![Forks Count](https://img.shields.io/github/forks/naaive/orange.svg?style=flat)](https://github.com/naaive/orange/network/members)
[![LICENSE](https://img.shields.io/badge/license-gpl-green?style=flat)](https://github.com/naaive/orange/blob/master/LICENSE)

[![Windows Support](https://img.shields.io/badge/Windows-0078D6?style=flat&logo=windows&logoColor=white)](https://github.com/naaive/orange/releases)
[![Windows Support](https://img.shields.io/badge/MACOS-adb8c5?style=flat&logo=macos&logoColor=white)](https://github.com/naaive/orange/releases)
[![Linux Support](https://img.shields.io/badge/linux-1793D1?style=flat&logo=linux&logoColor=white)](https://github.com/naaive/orange/releases)
</div>

## 介绍

![Demo](screenshot/orange_0.0.5.gif)

Orange是一款跨平台的**文件搜索**工具。

## ✨特点
- 使用简单，自带中文分词、拼音、补全
- 毫秒级搜索响应
- 低CPU、内存资源占用
- 实时监听文件变化
- 轻量安装包
- 简单大方UI

## 编译 
- 搭建Tauri开发环境 (https://tauri.studio/docs/getting-started/setting-up-macos)
- 运行 `yarn`
- 运行 `yarn build`
- 运行 `yarn tauri-build`


## 下载

点击 [release page](https://github.com/naaive/orange/releases).

## 提示

如果你是Macos用户, 安装后, 请执行`xattr -cr /Applications/Orange.app` 来修复 `“Orange” is damaged and can’t be opened. You should move it to the Trash.`问题。


## 架构
![arch](doc/img.png)


## 感谢
- Tauri https://tauri.studio
- Notify https://github.com/notify-rs/notify
- React https://github.com/facebook/react
- Tantivy https://github.com/quickwit-oss/tantivy
- Kv https://github.com/zshipko/rust-kv


## LICENSE

[GPL](https://github.com/naaive/orange/blob/master/LICENSE)



