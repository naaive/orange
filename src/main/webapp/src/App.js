import './App.css';
import {Button, Pane, Text, majorScale, Autocomplete, TextInput} from 'evergreen-ui'
import TabCmpt from "./TabCmpt";
import React from "react";

function App() {
    return (
        <div className="App">
            <Autocomplete
                title="Custom title"
                onChange={changedItem => console.log(changedItem)}
                items={['Apple', 'Apricot', 'Banana', 'Cherry', 'Cucumber']}
            >
                {({
                      key,
                      getInputProps,
                      getToggleButtonProps,
                      getRef,
                      inputValue,
                      openMenu,
                      toggleMenu
                  }) => (
                    <Pane key={key} ref={getRef} display="flex">
                        <TextInput
                            flex="1"
                            placeholder="Many Options!"
                            value={inputValue}
                            onFocus={openMenu}
                            {...getInputProps()}
                        />
                        <Button onClick={toggleMenu} {...getToggleButtonProps()}>
                            Search
                        </Button>
                    </Pane>
                )}
            </Autocomplete>

           <div className="tab">
               <TabCmpt></TabCmpt>
           </div>
        </div>
    );
}

export default App;
