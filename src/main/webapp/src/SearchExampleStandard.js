import _ from 'lodash'
import faker from 'faker'
import React from 'react'
import {Grid, Search} from 'semantic-ui-react'
import * as R from "ramda";

const source = _.times(5, () => ({
    title: faker.company.companyName(),
    // description: faker.company.catchPhrase()
}))

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

            const re = new RegExp(_.escapeRegExp(data.value), 'i')
            const isMatch = (result) => re.test(result.title)

            let resp = await fetch(`http://localhost:8080/sg?kw=${encodeURI(data.value)}`);
            let json = await resp.json();
            let titles = R.map(
                x => ({title: x})
            )(top6(json));
            dispatch({
                type: 'FINISH_SEARCH',
                results: titles,
            })
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
                        console.log(123)
                        dispatch({
                            type: 'UPDATE_SELECTION',
                            selection: {doTxtChange: doTxtChange, title: data.result.title}
                        })
                    }

                    }
                    onSearchChange={handleSearchChange}
                    results={results}
                    value={value}
                />
            </Grid.Column>
        </>
    )
}

export default SearchExampleStandard