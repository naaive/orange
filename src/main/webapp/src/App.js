import React, {createRef, useCallback, useEffect, useState} from "react";
import _ from "lodash";
import SearchExampleStandard from "./SearchExampleStandard";
import TableExampleCollapsing from "./TableExampleCollapsing";

const R = require('ramda');


function App() {


    const [ref, setRef] = useState();
    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [kw, setKw] = useState('');

    useEffect(() => {
        doTxtChange('c')
        setRef(createRef());
    }, []);

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
                <div className="search">
                    <SearchExampleStandard/>
                </div>
            <div className="oitems">
                <TableExampleCollapsing items={items}/>
            </div>
        </div>
    );
}

export default App;
