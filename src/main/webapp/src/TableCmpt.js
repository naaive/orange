import {Pane, Table} from 'evergreen-ui'
import React from "react";
import './TableCmpt.css'
import moment from 'moment';
import File from './file.svg'
import Folder from './folder.svg'
import OpenFolder from './open_folder.svg'
import Copy from './copy.svg'

const R = require('ramda');

function TableCmpt({items}) {
    function bytesToSize(bytes) {
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
        if (bytes === 0) return '0 Byte';
        const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
        return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i];
    }

    return <div className="table">
        <Table.Body>
            <Table.Head>
                <Table.TextCell>
                    name
                </Table.TextCell>
                <Table.TextCell flexShrink={0} flexGrow={3}>path</Table.TextCell>
                <Table.TextCell>modifiedAt</Table.TextCell>
                <Table.TextCell>size</Table.TextCell>
                <Table.TextCell>OP</Table.TextCell>
            </Table.Head>
            <Table.Body>
                {
                    R.map(x => {
                        let isDir = R.prop('isDir')(x);
                        let prop = R.prop('absPath')(x);
                        let split = R.split('\\')(prop);
                        let tail = R.last(split);
                        return <>
                            <Table.Row>
                                <Table.TextCell>
                                    <div className="icon">
                                        {isDir ? <img src={Folder}/> :
                                            <img src={File}/>}
                                        {tail}
                                    </div>

                                </Table.TextCell>
                                <Table.TextCell flexShrink={0} flexGrow={3}>
                                    {R.prop('absPath', x)}
                                </Table.TextCell>
                                <Table.TextCell>{moment(R.prop('modifiedAt', x)).format("YYYY-MM-DD h:mm:ss")}</Table.TextCell>
                                <Table.TextCell>{bytesToSize(R.prop('size', x))}</Table.TextCell>
                                <Table.TextCell>
                                    <Pane display="flex" alignItems="center">
                                        <div className="icon">
                                            <img src={OpenFolder} alt={'openfolder'}/>}
                                        </div>
                                        <div className="icon">
                                            <img src={Copy} alt={'openfolder'}/>}
                                        </div>
                                    </Pane>
                                </Table.TextCell>
                            </Table.Row>
                        </>;
                    }, items)
                }


            </Table.Body>
        </Table.Body>
    </div>
}

export default TableCmpt;