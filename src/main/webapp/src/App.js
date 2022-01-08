import React, {useCallback, useEffect, useState} from "react";
import _ from "lodash";
import SearchExampleStandard from "./SearchExampleStandard";
import TableExampleCollapsing from "./TableExampleCollapsing";

import * as R from "ramda";

function App() {


    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [kw, setKw] = useState('');

    useEffect(() => {
        doTxtChange('吾爱破解工具')
    }, []);

    function top6(json) {
        return R.pipe(R.map(R.prop('name')), R.take(6))(json);
    }

    async function doTxtChange(v) {
        let resp = await fetch(`http://localhost:41320/q?kw=${encodeURI(v)}`);
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
        let resp = await fetch(`http://localhost:41320/q?kw=${encodeURI(kw)}`);
        let json = await resp.json();
        setItems(json);
        setSuggestions(top6(json));
    }


    return (
        <div className="App">
            <div className="search">
                <SearchExampleStandard setItems={setItems} doTxtChange={doTxtChange}/>
            </div>
            <div className="oitems">
                <TableExampleCollapsing items={items}/>
            </div>
        </div>
    );
}

export default App;
