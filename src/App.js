import * as React from 'react';
import './App.css';
import {PivotIconCountExample} from "./PivotIconCountExample";
import Items from "./Items";
import {Scrollbars} from 'react-custom-scrollbars';
import SearchBox from "./SearchBox";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api";
import {search} from "./utils";



const App = () => {
    const [items, setItems] = useState([]);

    return (
        <div>

            <PivotIconCountExample/>
            <div className="search-box">
                <SearchBox setItems={setItems}/>
            </div>
            <div className="items">

                <Scrollbars autoHide autoHideTimeout={500}
                            autoHideDuration={200}>
                    <Items  items={items} setItems={setItems}/>
                </Scrollbars>


            </div>
        </div>
    );
};

export default App;
