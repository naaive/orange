import * as React from 'react';
import {useEffect, useState} from 'react';
import Items from "./Items";
import {Scrollbars} from 'react-custom-scrollbars';
import SearchBox from "./SearchBox";
import {Dialog, DialogFooter, DialogType, PrimaryButton, ProgressIndicator} from "@fluentui/react";
import {change_lang, get_lang, walk_metrics} from "./utils";
import {appWindow} from '@tauri-apps/api/window'
import {useBoolean} from '@fluentui/react-hooks';
import Tab from "./Tab";
import {useTranslation} from "react-i18next";
import i18next from "i18next";


const dialogContentProps = {
    type: DialogType.normal,
    title: 'Tip',
    closeButtonAriaLabel: 'Close',
    subText: 'It will be reindexed on next boot!',
};


const App = ({setTheme,theme}) => {

    const [items, setItems] = useState([]);
    const [kw, setKw] = useState('');
    const [selectedKey, setSelectedKey] = useState(0);
    const [hideDialog, {toggle: toggleHideDialog}] = useBoolean(true);
    let [init, setInit] = useState(false);
    let [lang, setLang] = useState(false);
    let [percent, setPercent] = useState(0);
    let [totalFiles, setTotalFiles] = useState(0);

    const { t } = useTranslation();
    useEffect(() => {
        if (!init) {

            get_lang().then(lang => {
                if (lang==="default") {
                    let localeLang= navigator.language || navigator.userLanguage;
                    let _ = i18next.changeLanguage(localeLang, (err, t) => {
                        if (err) return console.log('something went wrong loading', err);
                        t('key');
                    });
                    setLang(localeLang)
                } else {
                    let _ = i18next.changeLanguage(lang, (err, t) => {
                        if (err) return console.log('something went wrong loading', err);
                        t('key');
                    });
                    setLang(lang);
                }
            })

            setInterval(()=>{
                walk_metrics().then(({percent,total_files}) => {
                    console.log(123)
                    setPercent(percent);
                    setTotalFiles(total_files);
                })
            },2000)

            appWindow.listen('reindex', ({event, payload}) => {
                if (hideDialog) {
                    toggleHideDialog();
                }
            })
            setInit(true);
        }
    }, [init, hideDialog]);

    return (
        <div className={"body"}>
            <Dialog
                hidden={hideDialog}
                onDismiss={toggleHideDialog}
                dialogContentProps={dialogContentProps}
            >
                <DialogFooter>
                    <PrimaryButton onClick={toggleHideDialog} text="Confirm"/>
                    {/*<DefaultButton onClick={toggleHideDialog} text="Don't send" />*/}
                </DialogFooter>
            </Dialog>
            <Tab lang={lang} setLang={setLang} setTheme={setTheme} theme={theme} selectedKey={selectedKey} kw={kw} setItems={setItems} setSelectedKey={setSelectedKey}/>
            <div className="search-box">
                <SearchBox kw={kw} setKw={setKw} setItems={setItems} selectedKey={selectedKey}/>
            </div>
            <div className="items">

                <Scrollbars autoHide autoHideTimeout={500}
                            autoHideDuration={200}>
                    <Items kw={kw} items={items} setItems={setItems}/>
                </Scrollbars>
            </div>
            <div className="status-bar">

                <div className="progress">
                    <div className={"name"}>
                        {t("progress")}
                    </div>
                    <div className={"cmpt"}>
                        <ProgressIndicator percentComplete={percent/100} />
                    </div>
                </div>
                <div className="files">
                    {t("file-indexed")}{totalFiles.toLocaleString()}
                </div>
                {/*<div className="files">*/}
                {/*    V{version}*/}
                {/*</div>*/}
            </div>
        </div>
    );
};

export default App;
