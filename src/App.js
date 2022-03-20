import React, {useEffect, useState} from "react";
import Search from "./SearchBox";

import * as R from "ramda";
import {invoke} from "@tauri-apps/api";
import {Scrollbars} from 'react-custom-scrollbars';
import Items from "./Items";
import {ToastContainer, toast} from 'react-toastify';
import {Zoom} from 'react-toastify';
import {FormControl, HStack, Radio, RadioGroup} from "@chakra-ui/react";

function App() {


    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [kw, setKw] = useState('');

    useEffect(() => {

        setTimeout(() => doTxtChange('*'), 200)

        let run = 0;
        let handler;
        handler = setInterval(() => {
            if (items.length === 0 && run === 0) {
                run = 1;
                doTxtChange('*').then(() => {
                    if (items.length !== 0) {
                        clearInterval(handler);
                    }
                })
            }
        }, 200);

        let done=false;
        let toastId = toast.loading("0 files are indexed...");


        setInterval(()=>{
            invoke("walk_metrics").then(({percent, total_files}) => {
                console.log(percent)
                if (percent === 100) {
                    toast.update(toastId, { render: `${total_files} files indexed`, type: "success", isLoading: false });
                    if (!done) {
                        setTimeout(function () {
                            toast.dismiss(toastId);
                        },1000)
                    }
                    done = true;
                } else {
                    toast.update(toastId, { render: `${total_files} files indexed`, type: "success", isLoading: true });
                }
            })
        },1000)
    }, []);

    function top6(json) {
        return R.pipe(R.map(R.prop('name')), R.take(6))(json);
    }

    async function doTxtChange(v) {
        setKw(v);
        invoke('my_custom_command', {
            number: 0,
            kw: v
        })
            .then((res) => {
                    setItems(res.file_views);
                    setSuggestions(top6(res.file_views));
                }
            )
            .catch((e) => console.error(e))

    }



    return (

            <div className="App" >


               <div className="header">
                   <div className="search-box">
                       <Search setItems={setItems} doTxtChange={doTxtChange}/>
                   </div>
                   <div className={"filter"}>
                       <FormControl as='fieldset'>
                           <RadioGroup defaultValue='Itachi'>
                               <HStack spacing='24px'>
                                   <Radio  colorScheme='orange' value='All'>All</Radio>
                                   <Radio colorScheme='orange' value='Image'>Image</Radio>
                                   <Radio colorScheme='orange' value='Video'>Video</Radio>
                                   <Radio colorScheme='orange' value='Code'>Code</Radio>
                               </HStack>
                           </RadioGroup>
                       </FormControl>
                   </div>
               </div>


            <div className="items">

                <Scrollbars autoHide autoHideTimeout={500}
                            autoHideDuration={200}>
                    <Items items={items}/>


                </Scrollbars>


                </div>
                <ToastContainer position="bottom-center"
                                hideProgressBar={false}
                                theme={"light"}
                                limit={1}
                                transition={Zoom}
                />
            </div>

    );
}

export default App;
