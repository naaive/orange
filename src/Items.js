import React from 'react'
import * as R from "ramda";
import moment from "moment";
import {defaultStyles, FileIcon} from 'react-file-icon';
import Folder from "./folder.svg";
import {invoke} from "@tauri-apps/api";


function bytesToSize(bytes) {
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 Byte';
    const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
    return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i];
}

function Items({items, kw}) {

    function handleClick(absPath) {
        invoke('my_custom_command', {
            number: 1,
            kw:absPath
        })
    }

    return <Table compact unstackable selectable basic='very' size='small'>

        <Table.Header>
            <Table.Row>
                <Table.HeaderCell>Name</Table.HeaderCell>
                <Table.HeaderCell>Size</Table.HeaderCell>
                <Table.HeaderCell>Last Modified</Table.HeaderCell>
                <Table.HeaderCell>Path</Table.HeaderCell>
            </Table.Row>
        </Table.Header>

        <Table.Body>
            {
                R.map(x => {
                    let isDir = R.prop('is_dir')(x);
                    let absPath = R.prop('abs_path', x);

                    let name = R.prop("name")(x);

                    const extSplit = R.split('.');
                    let ext = R.last(extSplit(name));

                    return <Table.Row key={absPath} onDoubleClick={() => handleClick(absPath)}>
                        <Table.Cell>
                            <Header as='h5' image>
                                <div className="icon">
                                    {
                                        isDir ? <img src={Folder}/> :
                                            <FileIcon extension={ext} {...defaultStyles[ext]} />
                                    }

                                </div>
                                <Header.Content>
                                    {name}
                                </Header.Content>
                            </Header>
                        </Table.Cell>
                        <Table.Cell>

                            {bytesToSize(R.prop('size')(x))}
                        </Table.Cell>
                        <Table.Cell>
                            {moment(R.prop('mod_at', x)).format("YYYY-MM-DD h:mm:ss")}
                        </Table.Cell>
                        <Table.Cell>
                            {absPath}
                        </Table.Cell>

                    </Table.Row>
                })(items)
            }

        </Table.Body>
    </Table>;
}

export default Items