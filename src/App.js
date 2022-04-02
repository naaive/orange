import * as React from 'react';
import {useEffect, useState} from 'react';
import './App.css';
import Items from "./Items";
import {Scrollbars} from 'react-custom-scrollbars';
import SearchBox from "./SearchBox";
import {Dialog, DialogFooter, DialogType, PrimaryButton} from "@fluentui/react";
import {change_lang, get_lang} from "./utils";
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
                    setSelectedKey(localeLang)
                } else {
                    let en = "en";
                    setSelectedKey(en);
                    change_lang(en)
                }
            })

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
            <Tab setTheme={setTheme} theme={theme} selectedKey={selectedKey} kw={kw} setItems={setItems} setSelectedKey={setSelectedKey}/>
            <div className="search-box">
                <SearchBox kw={kw} setKw={setKw} setItems={setItems} selectedKey={selectedKey}/>
            </div>
            <div className="items">

                <Scrollbars autoHide autoHideTimeout={500}
                            autoHideDuration={200}>
                    <Items kw={kw} items={items} setItems={setItems}/>
                </Scrollbars>
            </div>
        </div>
    );
};

export default App;
