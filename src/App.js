import React, {useEffect, useState} from "react";
import Search from "./SearchBox";

import * as R from "ramda";
import {invoke} from "@tauri-apps/api";
import {Scrollbars} from 'react-custom-scrollbars';
import Items from "./Items";
import {ToastContainer, toast} from 'react-toastify';
import {Zoom} from 'react-toastify';
import {
    Checkbox,
    Divider,
    FormControl,
    HStack, Input,
    InputGroup,
    InputLeftAddon, InputRightAddon,
    Radio,
    RadioGroup,
    Stack,
    Text
} from "@chakra-ui/react";

const fileType2ext={
    "image":"tif tiff bmp jpg gif png eps raw cr2 nef orf sr2 jpeg",
    "video":"mp4 mov wmv avi avchd flv f4v swf mkv",
    "document":"doc txt pdf ppt pptx docx xlsx xls"
}

function App() {


    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [kw, setKw] = useState('');
    const [isDir, setIsDir] = useState(true);
    const [isFile, setIsFile] = useState(true);
    const [ext, setExt] = useState(0);
    let [extText,setExtText] = useState("");
    let [pathText,setPathText] = useState("");


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
                // console.log(percent)
                if (percent === 100) {
                    toast.update(toastId, { render: `${total_files} files indexed`, type: "success", isLoading: false });
                    if (!done) {
                        setTimeout(function () {
                            toast.dismiss(toastId);
                        },2000)
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

    async function doTxtChange(kw,is_dir_opt,ext_opt,parent_dirs_opt) {
        setKw(kw);
        let isDirOpt;
        if (isDir && isFile) {
            isDirOpt = undefined;
        }else if (isDir){
            isDirOpt = true;
        }else if (isFile) {
            isDirOpt = false;
        }
        let extOpt;
        if (ext === 0) {
            extOpt = undefined;
        }else if (ext === 1) {
            extOpt= fileType2ext["image"]
        }else if (ext === 2) {
            extOpt= fileType2ext["video"]
        }else if (ext === 3) {
            extOpt= fileType2ext["document"]
        }
        if (!R.isEmpty(extText)) {
            extOpt = extText;
        }
        if (R.isEmpty(pathText)) {
            pathText = undefined;
        }

        invoke('my_custom_command', {
            number: 0,
            kw: kw,
            isDirOpt:isDirOpt,
            extOpt: extOpt,
            parentDirsOpt: pathText,
        })
            .then((res) => {
                    setItems(res.file_views);
                    setSuggestions(top6(res.file_views));
                }
            )
            .catch((e) => console.error(e));

    }



    return (

            <div className="App" >


               <div className="header">
                   <div className="search-box">
                       <Search setItems={setItems} doTxtChange={doTxtChange}/>
                   </div>
                   <div className={"filter"}>

                       <FormControl as='fieldset'>

                           <RadioGroup value={ext}>
                               <HStack spacing='24px'>
                                   <Checkbox  colorScheme='gray' isChecked={isDir} onChange={()=> {
                                       if (isDir && !isFile) {
                                           return
                                       }
                                       setIsDir(!isDir);
                                   }}>
                                       Folder
                                   </Checkbox>
                                   <Checkbox  colorScheme='gray' isChecked={isFile} onChange={()=> {
                                       if (isFile && !isDir) {
                                           return
                                       }
                                       setIsFile(!isFile)
                                   }}>
                                       File
                                   </Checkbox>


                                   <span onClick={()=>setExt(0)}>
                                     <Radio  colorScheme='gray' value={0}  >All</Radio>
                                   </span>
                                   <span onClick={()=>setExt(1)}>
                                    <Radio  colorScheme='gray' value={1}>Image</Radio>
                                   </span>
                                   <span onClick={()=>setExt(2)}>
                                      <Radio  colorScheme='gray' value={2}>Video</Radio>
                                   </span>
                                   <span onClick={()=>setExt(3)}>
                                    <Radio  colorScheme='gray' value={3}>Doc</Radio>
                                   </span>
                                   <InputGroup size='sm'>
                                       <InputLeftAddon children='Type' />
                                       <Input placeholder='pdf' value={extText} onInput={(e)=>setExtText(e.target.value)}/>
                                   </InputGroup>
                                   <InputGroup size='sm'>
                                       <InputLeftAddon children='Path' />
                                       <Input placeholder='/' value={pathText}/>
                                   </InputGroup>

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
