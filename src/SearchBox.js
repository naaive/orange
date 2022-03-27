import React, {useEffect, useState} from 'react';
import {TagPicker} from "@fluentui/react";
import {invoke} from "@tauri-apps/api";
import * as R from "ramda";
import {search} from "./utils";

function top6(json) {
    return R.take(6)(json);
}

async function filterSuggestedTags(filter, selectedItems) {
    let res = await invoke('my_custom_command', {
        number: 2,
        kw: filter
    });

    let titles = R.map(x => ({name: x, key: x}))(R.uniq(R.map(
        x => (x.name)
    )(res.file_views)));

    if (titles[0]) {
        if (titles[0].name !== filter) {
            titles.unshift({name: filter, key: filter})
        }
    }

    return top6(titles);
}

const SearchBox = ({setItems, kw, setKw, selectedKey}) => {
    let [init, setInit] = useState(false);
    let [handler, setHandler] = useState();
    useEffect(() => {
        let number = setInterval(async () => {
            if (!init) {
                let kw0 = "*";
                let items = await search(kw0, selectedKey);
                if (R.isEmpty(items) || R.isNil(items)) {
                    return;
                }
                setItems(items);
                setKw(kw0);
                setInit(true)
            }

        }, 1000);
        setHandler(number);

    }, [init])
    useEffect(() => {
        if (init) {
            clearInterval(handler);
        }
    }, [init])
    return (
        <div>
            <TagPicker
                onItemSelected={function (e) {
                    let kw0 = e.name;
                    setKw(kw0);
                    search(kw0, selectedKey).then(items => {
                        setItems(items);
                    });
                    return e;
                }}
                removeButtonAriaLabel="Remove"
                selectionAriaLabel="Selected colors"
                onResolveSuggestions={filterSuggestedTags}
                getTextFromItem={(item) => item.name}
                pickerSuggestionsProps={
                    {noResultsFoundText: "Non Exist File"}
                }
                enableSelectedSuggestionAlert={false}
                itemLimit={1}
                pickerCalloutProps={{doNotLayer: true}}
                inputProps={{
                    id: "pickerId",
                    value: "",
                    onKeyUp: onKeyUp
                }}
            />
        </div>
    );

    async function onKeyUp(event) {
        let keyCode = event.keyCode;
        if (keyCode === 13 || keyCode === 27) {
            let kw = event.target.value;
            document.body.click();
            setKw(kw);
            let items = await search(kw, selectedKey);
            setItems(items);
        }
    }
};

export default SearchBox;