import React from 'react'
import * as R from "ramda";
import {invoke} from "@tauri-apps/api";

import {AutoComplete, AutoCompleteInput, AutoCompleteItem, AutoCompleteList,} from "@choc-ui/chakra-autocomplete";
import {Icon, InputGroup, InputLeftElement,} from "@chakra-ui/react";

const initialState = {
    loading: false,
    results: [],
    value: '',
}

function exampleReducer(state, action) {
    switch (action.type) {
        case 'CLEAN_QUERY':
            return initialState
        case 'START_SEARCH':
            return {...state, loading: true, value: action.query}
        case 'FINISH_SEARCH':
            return {...state, loading: false, results: action.results}
        case 'UPDATE_SELECTION':
            let value = action.selection.title;
            action.selection.doTxtChange(value)
            return {...state, value: value}

        default:
            throw new Error()
    }
}

function top6(json) {
    return R.take(6)(json);
}

function SearchBox({setItems, doTxtChange}) {
    const [state, dispatch] = React.useReducer(exampleReducer, initialState)
    const {loading, results, value} = state

    const timeoutRef = React.useRef()
    const handleSearchChange = React.useCallback((e, data) => {
        clearTimeout(timeoutRef.current)
        dispatch({type: 'START_SEARCH', query: data.value})

        timeoutRef.current = setTimeout(async () => {
            if (data.value.length === 0) {
                dispatch({type: 'CLEAN_QUERY'})
                return
            }

            invoke('my_custom_command', {
                number: 2,
                kw: data.value
            })
                .then((res) => {
                    let titles = R.uniq(R.map(
                        x => ({title: x.name})
                    )(res.file_views));


                    dispatch({
                        type: 'FINISH_SEARCH',
                        results: top6(titles),
                    })
                    }
                )
                .catch((e) => console.error(e))
        }, 300)
    }, [])


    React.useEffect(() => {
        return () => {
            clearTimeout(timeoutRef.current)

        }
    }, [])

    const options = ["apple", "appoint", "zap", "cap", "japan"];

    return (
        <>
            <AutoComplete rollNavigation>
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
                    <AutoCompleteInput variant="filled" placeholder="Search..." />
                </InputGroup>
                <AutoCompleteList>
                    {options.map((option, oid) => (
                        <AutoCompleteItem
                            key={`option-${oid}`}
                            value={option}
                            textTransform="capitalize"
                        >
                            {option}
                        </AutoCompleteItem>
                    ))}
                </AutoCompleteList>
            </AutoComplete>
        </>
    )
}

export default SearchBox