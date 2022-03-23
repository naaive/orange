import React, {useEffect, useState} from "react";
import Search from "./SearchBox";
import {PhoneIcon, AddIcon, WarningIcon, ArrowRightIcon} from '@chakra-ui/icons'

import * as R from "ramda";
import {invoke} from "@tauri-apps/api";
import {Scrollbars} from 'react-custom-scrollbars';
import Items from "./Items";
import {ToastContainer, toast} from 'react-toastify';
import {Zoom} from 'react-toastify';
import {
    ButtonGroup,
    Checkbox,
    Divider,
    FormControl,
    HStack, IconButton, Input,
    InputGroup,
    InputLeftAddon, InputRightAddon,
    Radio,
    RadioGroup,
    Stack,
    Text
} from "@chakra-ui/react";
// import component 👇
import Drawer from 'react-modern-drawer'

//import styles 👇
import 'react-modern-drawer/dist/index.css'

const fileType2ext = {
    "image": "tif tiff bmp jpg gif png eps raw cr2 nef orf sr2 jpeg",
    "video": "mp4 mov wmv avi avchd flv f4v swf mkv",
    "document": "doc txt pdf ppt pptx docx xlsx xls"
}

function App() {

    const [isOpen, setIsOpen] = React.useState(false)
    const toggleDrawer = () => {
        setIsOpen((prevState) => !prevState)
    }

    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [kw, setKw] = useState('');
    const [isDir, setIsDir] = useState(true);
    const [isFile, setIsFile] = useState(true);
    const [ext, setExt] = useState(0);
    let [extText, setExtText] = useState("");


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

        let done = false;
        let toastId = toast.loading("0 files are indexed...");


        setInterval(() => {
            invoke("walk_metrics").then(({percent, total_files}) => {
                // console.log(percent)
                if (percent === 100) {
                    toast.update(toastId, {render: `${total_files} files indexed`, type: "success", isLoading: false});
                    if (!done) {
                        setTimeout(function () {
                            toast.dismiss(toastId);
                        }, 2000)
                    }
                    done = true;
                } else {
                    toast.update(toastId, {render: `${total_files} files indexed`, type: "success", isLoading: true});
                }
            })
        }, 1000)
    }, []);

    function top6(json) {
        return R.pipe(R.map(R.prop('name')), R.take(6))(json);
    }

    async function doTxtChange(kw) {
        setKw(kw);
        let isDirOpt;
        if (isDir && isFile) {
            isDirOpt = undefined;
        } else if (isDir) {
            isDirOpt = true;
        } else if (isFile) {
            isDirOpt = false;
        }
        let extOpt;
        if (ext === 0) {
            extOpt = undefined;
        } else if (ext === 1) {
            extOpt = fileType2ext["image"]
        } else if (ext === 2) {
            extOpt = fileType2ext["video"]
        } else if (ext === 3) {
            extOpt = fileType2ext["document"]
        }
        if (!R.isEmpty(extText)) {
            extOpt = extText;
        }

        invoke('my_custom_command', {
            number: 0,
            kw: kw,
            isDirOpt: isDirOpt,
            extOpt: extOpt,
            parentDirsOpt: undefined,
        })
            .then((res) => {
                    setItems(res.file_views);
                    setSuggestions(top6(res.file_views));
                }
            )
            .catch((e) => console.error(e));

    }


    return (

        <div className="App">
            <Drawer
                open={isOpen}
                onClose={()=>{
                    doTxtChange(kw)
                    toggleDrawer()
                }}
                duration={200}
                size={260}
                direction='left'
                className='bla bla bla'
            >
                <div className={"filter"}>

                    <div className="head">
                        Selector
                    </div>
                    <FormControl as='fieldset'>

                        <RadioGroup value={ext}>
                            <div className="title">
                                Is Dir
                            </div>
                            <HStack >
                                <Checkbox colorScheme='gray' isChecked={isFile} onChange={() => {
                                    if (isFile && !isDir) {
                                        return
                                    }
                                    setIsFile(!isFile)
                                }}>
                                    File
                                </Checkbox>
                                <Checkbox colorScheme='gray' isChecked={isDir} onChange={() => {
                                    if (isDir && !isFile) {
                                        return
                                    }
                                    setIsDir(!isDir);
                                }}>
                                    Folder
                                </Checkbox>



                            </HStack>
                            <div className="title">
                               Type
                            </div>
                            <HStack spacing='12px'>
                            <span onClick={() => setExt(0)}>
                                     <Radio colorScheme='gray' value={0}>All</Radio>
                                   </span>
                                <span onClick={() => setExt(1)}>
                                    <Radio colorScheme='gray' value={1}>Image</Radio>
                                   </span>
                                <span onClick={() => setExt(2)}>
                                      <Radio colorScheme='gray' value={2}>Video</Radio>
                                   </span>
                                <span onClick={() => setExt(3)}>
                                    <Radio colorScheme='gray' value={3}>Doc</Radio>
                                   </span>
                            </HStack>
                            <div className="title">
                                Ext
                            </div>
                            <HStack>
                                <InputGroup size='sm'>
                                    <Input placeholder='pdf' value={extText}
                                           onInput={(e) => setExtText(e.target.value)}/>
                                </InputGroup>
                            </HStack>
                        </RadioGroup>
                    </FormControl>
                </div>
            </Drawer>

            <div className="header">
                <div className="search-box">
                    <Search setItems={setItems} doTxtChange={doTxtChange}/>
                    <IconButton className={'btn'} size={'sm'} aria-label='Search database' onClick={toggleDrawer} icon={<AddIcon />} />

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
