import React, {useEffect, useState} from "react";
import Search from "./SearchBox";

import * as R from "ramda";
import {invoke} from "@tauri-apps/api";
import {Scrollbars} from 'react-custom-scrollbars';
import Items from "./Items";

function App() {


    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [kw, setKw] = useState('');

    useEffect(() => {
        setTimeout(() => doTxtChange('*'), 200)

        let run = 0;
        let handler;
        handler = setInterval(() => {
            if (items.length === 0 && run === 0) {
                run = 1;
                doTxtChange('*').then(() => {
                    if (items.length !== 0) {
                        clearInterval(handler);
                    }
                })
            }
        }, 200);
    }, []);

    function top6(json) {
        return R.pipe(R.map(R.prop('name')), R.take(6))(json);
    }

    async function doTxtChange(v) {
        setKw(v);
        invoke('my_custom_command', {
            number: 0,
            kw: v
        })
            .then((res) => {
                    setItems(res.file_views);
                    setSuggestions(top6(res.file_views));
                }
            )
            .catch((e) => console.error(e))

    }


    return (

            <div className="App" >
                <div className="search-box">
                    <Search setItems={setItems} doTxtChange={doTxtChange}/>
                </div>

                <div className="items">

                    <Scrollbars autoHide     autoHideTimeout={500}
                                autoHideDuration={200} >
                    <Items items={items}/>


                    </Scrollbars>


                </div>
            </div>

    );
}

export default App;
