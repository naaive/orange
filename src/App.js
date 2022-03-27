import * as React from 'react';
import './App.css';
import {PivotIconCountExample} from "./PivotIconCountExample";
import Items from "./Items";
import {Scrollbars} from 'react-custom-scrollbars';
import SearchBox from "./SearchBox";



const App=() => {
  return (
    <div>

        <PivotIconCountExample></PivotIconCountExample>
        <div className="search-box">
            <SearchBox/>
            {/*<TagPickerInlineExample></TagPickerInlineExample>*/}
        </div>
        <div className="items">

            <Scrollbars autoHide autoHideTimeout={500}
                        autoHideDuration={200}>
                <Items/>
            </Scrollbars>


        </div>
        {/*<DetailsListDocumentsExample></DetailsListDocumentsExample>*/}
    </div>
  );
};

export default App;
