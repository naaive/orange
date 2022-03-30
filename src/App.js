import * as React from 'react';
import {useEffect, useState} from 'react';
import './App.css';
import Items from "./Items";
import {Scrollbars} from 'react-custom-scrollbars';
import SearchBox from "./SearchBox";
import {Pivot, PivotItem, PrimaryButton} from "@fluentui/react";
import {search} from "./utils";
import { appWindow } from '@tauri-apps/api/window'
import { Dialog, DialogType, DialogFooter } from '@fluentui/react/lib/Dialog';
import { useId, useBoolean } from '@fluentui/react-hooks';



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
    const [hideDialog, { toggle: toggleHideDialog }] = useBoolean(true);
    let [init,setInit] = useState(false);

    useEffect(() => {
        if (!init) {
            appWindow.listen('reindex', ({event, payload}) => {
                if (hideDialog) {
                    toggleHideDialog();
                }
            })
            setInit(true);
        }
    }, [init,hideDialog]);

    return (
        <div>
            <Dialog
                hidden={hideDialog}
                onDismiss={toggleHideDialog}
                dialogContentProps={dialogContentProps}
            >
                <DialogFooter>
                    <PrimaryButton onClick={toggleHideDialog} text="Confirm" />
                    {/*<DefaultButton onClick={toggleHideDialog} text="Don't send" />*/}
                </DialogFooter>
            </Dialog>
            <Pivot aria-label="Count and Icon Pivot Example" selectedKey={String(selectedKey)} onLinkClick={(event) => {
                let key = event.key.substr(1);
                setSelectedKey(key)
                search(kw, key).then(value => {
                    setItems(value)
                })
            }}>
                {/*https://uifabricicons.azurewebsites.net/?help*/}
                <PivotItem headerText="All" itemIcon="ViewAll2" itemKey="0">
                </PivotItem>
                <PivotItem headerText="Folder" itemIcon="FabricFolder" itemKey="1">
                </PivotItem>
                <PivotItem headerText="Doc" itemIcon="Document" itemKey="2">
                </PivotItem>
                <PivotItem headerText="Video" itemIcon="Video" itemKey="3">
                </PivotItem>
                <PivotItem headerText="Photo" itemIcon="Photo2" itemKey="4">
                </PivotItem>
            </Pivot>
            <div className="search-box">
                <SearchBox kw={kw} setKw={setKw} setItems={setItems} selectedKey={selectedKey}/>
            </div>
            <div className="items">

                <Scrollbars autoHide autoHideTimeout={500}
                            autoHideDuration={200}>
                    <Items items={items} setItems={setItems}/>
                </Scrollbars>


            </div>
        </div>
    );
};

export default App;
