import React from 'react';
import {TagPicker} from "@fluentui/react";

function filterSuggestedTags(filter, selectedItems){


    return [];
}
const SearchBox = () => {
    return (
        <div>
            <TagPicker
                removeButtonAriaLabel="Remove"
                selectionAriaLabel="Selected colors"
                onResolveSuggestions={filterSuggestedTags}
                // getTextFromItem={getTextFromItem}
                pickerSuggestionsProps={
                    {noResultsFoundText:"Non Exist File"}
                }
                itemLimit={1}
                pickerCalloutProps={{ doNotLayer: true }}
                inputProps={{
                    id: "pickerId",
                }}
            />
        </div>
    );
};

export default SearchBox;