import './App.css';
import {Autocomplete, Button, Pane, TextInput} from 'evergreen-ui'
import TabCmpt from "./TabCmpt";
import React, {useState} from "react";

const R = require('ramda');

function App() {
    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [kw, setKw] = useState('');

    async function handleClick(toggleMenu) {
        toggleMenu();
        let resp = await fetch(`http://localhost:3001/api/q?kw=${kw}`);
        let json = await resp.json();
        setItems(json)
        let map = R.map(R.prop('absPath'), json);
        setSuggestions(map)
    }

    async function handleTxtChange(v) {
        setKw(v)
        let resp = await fetch(`http://localhost:3001/api/q?kw=${kw}`);
        let json = await resp.json();
        let map = R.map(R.prop('absPath'), json);
        setSuggestions(map)
    }

    return (
        <div className="App">
            <Autocomplete
                title=""
                onChange={changedItem => handleTxtChange(changedItem)}
                items={ suggestions}
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
                            onChange={e => handleTxtChange(e.target.value)}
                            onFocus={openMenu}
                        />
                        <Button onClick={() => handleClick(toggleMenu)}>
                            Search
                        </Button>
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
