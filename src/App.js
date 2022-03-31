import * as React from 'react';
import {useEffect, useState} from 'react';
import './App.css';
import Items from "./Items";
import {Scrollbars} from 'react-custom-scrollbars';
import SearchBox from "./SearchBox";
import {IconButton, Pivot, PivotItem, PrimaryButton} from "@fluentui/react";
import {search} from "./utils";
import {appWindow} from '@tauri-apps/api/window'
import {Dialog, DialogType, DialogFooter} from '@fluentui/react/lib/Dialog';
import {useId, useBoolean} from '@fluentui/react-hooks';
import Tab from "./Tab";


const dialogContentProps = {
    type: DialogType.normal,
    title: 'Tip',
    closeButtonAriaLabel: 'Close',
    subText: 'It will be reindexed on next boot!',
};


const App = () => {
    const [items, setItems] = useState([]);
    const [kw, setKw] = useState('');
    const [selectedKey, setSelectedKey] = useState(0);
    const [hideDialog, {toggle: toggleHideDialog}] = useBoolean(true);
    let [init, setInit] = useState(false);

    useEffect(() => {
        if (!init) {
            appWindow.listen('reindex', ({event, payload}) => {
                if (hideDialog) {
                    toggleHideDialog();
                }
            })
            setInit(true);
        }
    }, [init, hideDialog]);

    return (
        <div>
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
            <Tab selectedKey={selectedKey} kw={kw} setItems={setItems} setSelectedKey={setSelectedKey}/>
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
