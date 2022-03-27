import * as React from 'react';
import {useEffect, useState} from 'react';
import './App.css';
import Items from "./Items";
import {Scrollbars} from 'react-custom-scrollbars';
import SearchBox from "./SearchBox";
import {Pivot, PivotItem} from "@fluentui/react";
import {search} from "./utils";


const App = () => {
    const [items, setItems] = useState([]);
    const [kw, setKw] = useState('');
    const [selectedKey, setSelectedKey] = useState(0);

    return (
        <div>
            <Pivot aria-label="Count and Icon Pivot Example" selectedKey={String(selectedKey)} onLinkClick={(event) => {
                let key = event.key.substr(1);
                setSelectedKey(key)
                search(kw,key).then(value => {
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
