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

const TableExampleCollapsing = ({items}) => (


    <Table basic='very'  size='small' >
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
                    let prop = R.prop('absPath')(x);
                    let split = R.split('\\')(prop);
                    let tail = R.last(split);
                    let exsplit = R.split(".")(tail);
                    let ext = R.length(exsplit) > 1 ? R.last(exsplit) : '';
                    return <>
                        <Table.Row>
                            <Table.Cell>
                                <Header as='h4' image>
                                    <div className="icon">
                                        {
                                            isDir ? <img src={Folder}/> :
                                                <FileIcon extension={ext} {...defaultStyles[ext]} />
                                        }

                                    </div>
                                    <Header.Content>
                                        {tail}
                                        {/*<Header.Subheader>*/}
                                        {/*    {'size: ' +bytesToSize( R.prop('size')(x))}*/}
                                        {/*    <br/>*/}
                                        {/*    {'modifiedAt: ' + moment(R.prop('modifiedAt', x)).format("YYYY-MM-DD h:mm:ss")}*/}
                                        {/*</Header.Subheader>*/}
                                    </Header.Content>
                                </Header>
                            </Table.Cell>
                            <Table.Cell>{bytesToSize(R.prop('size')(x))}</Table.Cell>
                            <Table.Cell>{moment(R.prop('modifiedAt', x)).format("YYYY-MM-DD h:mm:ss")}</Table.Cell>
                            <Table.Cell>  {R.prop('absPath', x)}</Table.Cell>
                        </Table.Row>
                    </>

                })(items)
            }

        </Table.Body>
    </Table>
);

export default TableExampleCollapsing