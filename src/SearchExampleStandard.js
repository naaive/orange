import _ from 'lodash'
import faker from 'faker'
import React from 'react'
import {Grid, Search} from 'semantic-ui-react'
import * as R from "ramda";
import {invoke} from "@tauri-apps/api";


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

function SearchExampleStandard({setItems, doTxtChange}) {
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
                    let titles = R.map(
                        x => ({title: x.name})
                    )(top6(res.file_views));
                        console.log(titles)
                    dispatch({
                        type: 'FINISH_SEARCH',
                        results: titles,
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

    return (
        <>
            <Grid.Column width={12}>
                <Search
                    className="search-bar"
                    fluid={true}
                    loading={loading}
                    onResultSelect={(e, data) => {
                        dispatch({
                            type: 'UPDATE_SELECTION',
                            selection: {doTxtChange: doTxtChange, title: data.result.title}
                        })
                    }

                    }
                    onKeyUp={(event)=> {
                        if (event.keyCode === 13) {
                            doTxtChange(event.target.value)
                            // close suggest results
                            window.document.scripts[0].click()
                        }
                    }}
                    onSearchChange={handleSearchChange}
                    results={results}
                    value={value}
                />
            </Grid.Column>
        </>
    )
}

export default SearchExampleStandard