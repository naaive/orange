import React, {useState} from 'react'
import * as R from "ramda";
import {invoke} from "@tauri-apps/api";
import {AutoComplete, AutoCompleteInput, AutoCompleteItem, AutoCompleteList,} from "@choc-ui/chakra-autocomplete";
import {FormControl, HStack, Icon, InputGroup, InputLeftElement, Radio, RadioGroup,} from "@chakra-ui/react";


import _ from "lodash";

function top6(json) {
    return R.take(6)(json);
}

function SearchBox({setItems, doTxtChange}) {
    let [show, setShow] = useState(true);
    let [options, setOptions] = useState([]);

    return (
        <>
            <AutoComplete
                listAllValuesOnFocus={false}
                selectOnFocus={false}
                filter={() => true}
                closeOnBlur={true} rollNavigation onChange={v => {
                doTxtChange(v);
            }}

            >
                <InputGroup>
                    <InputLeftElement
                        pointerEvents="none"
                        color="inherit"
                        fontSize="1.2em"
                    >
                        <Icon boxSize="16px" viewBox="0 0 24 24" focusable="false">
                            <path
                                fill="currentColor"
                                d="M23.384,21.619,16.855,15.09a9.284,9.284,0,1,0-1.768,1.768l6.529,6.529a1.266,1.266,0,0,0,1.768,0A1.251,1.251,0,0,0,23.384,21.619ZM2.75,9.5a6.75,6.75,0,1,1,6.75,6.75A6.758,6.758,0,0,1,2.75,9.5Z"
                            ></path>
                        </Icon>
                    </InputLeftElement>
                    <AutoCompleteInput variant="filled" placeholder="Search..."
                                       onKeyUp={(event) => {

                                           let keyCode = event.keyCode;
                                           if (keyCode === 13) {
                                               doTxtChange(event.target.value)
                                               // close suggest results
                                               setShow(false)
                                           }
                                           if (keyCode === 27) {
                                               doTxtChange(event.target.value)

                                               setShow(false)
                                           }

                                       }}
                                       onChange={_.debounce(e => {
                                           setShow(true)
                                           invoke('my_custom_command', {
                                               number: 2,
                                               kw: e.target.value
                                           })
                                               .then((res) => {
                                                       let titles = R.uniq(R.map(
                                                           x => (x.name)
                                                       )(res.file_views));
                                                       console.log(titles)
                                                       setOptions(top6(titles));
                                                   }
                                               )
                                               .catch((e) => console.error(e))
                                       },150)}/>
                </InputGroup>
                {
                    show ? <AutoCompleteList>

                        {options.map((option, oid) => (
                            <AutoCompleteItem
                                key={`option-${oid}`}
                                value={option}
                                textTransform="capitalize"
                            >
                                {option}
                            </AutoCompleteItem>
                        ))}
                    </AutoCompleteList> : <></>
                }

            </AutoComplete>
        </>
    )
}

export default SearchBox