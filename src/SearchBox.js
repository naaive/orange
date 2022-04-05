import React, {useEffect, useState} from 'react';
import {TagPicker} from "@fluentui/react";
import * as R from "ramda";
import {search, suggest} from "./utils";

function top6(json) {
    return R.take(6)(json);
}


const SearchBox = ({setItems, kw, setKw, selectedKey,setTokenized}) => {
    let [init, setInit] = useState(false);
    let [handler, setHandler] = useState();
    useEffect(() => {
        let number = setInterval(async () => {
            if (!init) {
                let kw0 = "*";
                let {file_view,tokenized} = await search(kw0, selectedKey);
                if (R.isEmpty(file_view) || R.isNil(file_view)) {
                    return;
                }
                setItems(file_view);
                setTokenized(tokenized)
                setKw(kw0);
                setInit(true)
            }

        }, 200);
        setHandler(number);

    }, [init])
    useEffect(() => {
        if (init) {
            clearInterval(handler);
        }
    }, [init])

    async function filterSuggestedTags(filter, selectedItems) {
        let res = await suggest(kw);

        let titles = R.map(x => ({name: x, key: x}))(R.uniq(R.map(
            x => (x.name)
        )(res)));

        if (titles[0]) {
            if (titles[0].name !== kw) {
                titles.unshift({name: kw, key: kw})
            }
        }

        return top6(titles);
    }

    return (
        <div>
            <TagPicker
                onItemSelected={function (e) {
                    let kw0 = e.name;
                    setTimeout(()=>{
                        setKw(kw0);
                    },100)
                    search(kw0, selectedKey).then(({file_view,tokenized}) => {
                        setItems(file_view);
                        setTokenized(tokenized);
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
                    value: kw,
                    onKeyUp: onKeyUp
                }}
                onInputChange={(e)=>{
                        setKw(e);
                }}
            />
        </div>
    );

    async function onKeyUp(event) {
        let keyCode = event.keyCode;
        let kw0 = event.target.value;
        console.log("keyCode: "+keyCode+", kw0:"+kw0)
        setKw(kw0);
        if (keyCode === 13 || keyCode === 27) {

            document.body.click();
            console.log("onKeyUp")
            let {file_view,tokenized} = await search(kw0, selectedKey);
            setItems(file_view);
            setTokenized(tokenized);
        }
    }
};

export default SearchBox;