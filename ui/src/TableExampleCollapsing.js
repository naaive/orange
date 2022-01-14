import React from 'react'
import {Header, Table} from 'semantic-ui-react'
import * as R from "ramda";
import moment from "moment";
import {defaultStyles, FileIcon} from 'react-file-icon';
import Folder from "./folder.svg";


function bytesToSize(bytes) {
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 Byte';
    const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
    return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i];
}

function TableExampleCollapsing({items, kw}) {

    function handleClick(data) {
        fetch(`http://localhost:41320/ofd?kw=${encodeURI(data)}`);
    }

    return <Table unstackable selectable basic='very' size='small'>

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
                    let isDir = R.prop('isDir')(x);
                    let ext = R.prop('ext')(x)
                    let absPath = R.prop('absPath', x);
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
                                    {R.prop("name")(x)}
                                </Header.Content>
                            </Header>
                        </Table.Cell>
                        <Table.Cell>

                            {bytesToSize(R.prop('size')(x))}
                        </Table.Cell>
                        <Table.Cell>
                            {moment(R.prop('modifiedAt', x)).format("YYYY-MM-DD h:mm:ss")}
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

export default TableExampleCollapsing