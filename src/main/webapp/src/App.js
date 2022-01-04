import './App.css';
import {Autocomplete, Pane, TextInput} from 'evergreen-ui'
import TabCmpt from "./TabCmpt";
import React, {useCallback, useEffect, useState} from "react";
import _ from "lodash";

const R = require('ramda');


function App() {
    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [kw, setKw] = useState('');

    useEffect(() => {
        doTxtChange('c')
    },[]);

    function top6(json) {
        return R.pipe(R.map(R.prop('absPath')), R.take(6))(json);
    }

    async function doTxtChange(v) {
        let resp = await fetch(`http://localhost:8080/q?kw=${v}`);
        let json = await resp.json();
        setItems(json);
        setSuggestions(top6(json));
    }

    let debouncedTxtChange = useCallback(_.debounce(doTxtChange, 300), []);

    function onTextChange(v) {
        setKw(v);
        console.log(v)
        debouncedTxtChange(v);
    }


    async function handleClick(toggleMenu) {
        toggleMenu();
        let resp = await fetch(`http://localhost:8080/q?kw=${kw}`);
        let json = await resp.json();
        setItems(json);
        setSuggestions(top6(json));
    }


    return (
        <div className="App">
            <Autocomplete
                title=""
                onChange={changedItem => onTextChange(changedItem)}
                items={suggestions}
            >
                {({
                      key,
                      getRef,
                      openMenu,
                      toggleMenu
                  }) => (
                    <Pane key={key} ref={getRef} display="flex">
                        <TextInput
                            flex="1"
                            placeholder="Many Options!"
                            value={kw}
                            onChange={e => onTextChange(e.target.value)}
                            onFocus={openMenu}
                        />
                        {/*<Button onClick={() => handleClick(toggleMenu)}>*/}
                        {/*    Search*/}
                        {/*</Button>*/}
                    </Pane>
                )}
            </Autocomplete>

            <div className="tab">
                <TabCmpt items={items}/>
            </div>
        </div>
    );
}

export default App;
