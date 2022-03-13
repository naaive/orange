import React from 'react';
import DataTable from 'react-data-table-component';

import moment from "moment";
import * as R from "ramda";
import Folder from "./folder.svg";
import {defaultStyles, FileIcon} from "react-file-icon";
import {invoke} from "@tauri-apps/api";
import RightMenu from '@right-menu/react'
import copy from 'copy-to-clipboard';


const customStyles = {
    headRow: {
        style: {
            border: 'none',
        },
    },
    headCells: {
        style: {
            color: '#202124',
            fontSize: '13px',
        },
    },
    rows: {
        highlightOnHoverStyle: {
            backgroundColor: '#e8e8e8',
            borderBottomColor: '#FFFFFF ',
            borderRadius: '5px',
            outline: '1px solid #FFFFFF',
        },
    },
    pagination: {
        style: {
            border: 'none',
        },
    },
};
function options(row) {
    
    return [
        {
            type: 'li', 
            text: 'Open location', 
            callback: () => {
                open_file_location(row)
            }
        },
        {
            type: 'li', 
            text: 'Copy path', 
            callback: () => copy(row.abs_path) 
        },
    ]
}


function bytesToSize(bytes) {
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 Byte';
    const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
    return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i];
}


const columns = [
    {
        selector: row => {
            let isDir = R.prop('is_dir')(row);
            let name = R.prop("name")(row);
            const extSplit = R.split('.');
            let ext = R.last(extSplit(name));

            let icon = isDir ? <img src={Folder}/> :
                <FileIcon extension={ext} {...defaultStyles[ext]} />;
            return <>
                <div className="icon">
                    <span className={"img"}>
                    {icon}
                </span>
                </div>

            </>;
        },

        width: '50px', // custom width for icon button
        style: {
            padding: '0 16px !important',

borderBottom: '1px solid #FFFFFF',
            marginBottom: '-1px',
        },
    },
    {
        name: 'Name',
        cell: row =>
            <RightMenu theme="mac" options={options(row)} maxWidth={200} style={{cursor: "pointer"}}>
                <div className={"items-row"}>{row.name}</div>
            </RightMenu>,
        grow: 1,
        style: {
            color: '#202124',
            fontSize: '14px',
            fontWeight: 500,
        },
    },

    {

        name: 'Last Modified',
        width: '160px',
        cell: row => <RightMenu theme="mac" options={options(row)} maxWidth={200} >
            <div className={"items-row"}>{moment.unix(R.prop('mod_at')(row)).format("YYYY-MM-DD h:mm:ss")}</div>
        </RightMenu>,
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
    {
        name: 'Size',
        maxWidth: '80px',
        cell: row => <RightMenu theme="mac" options={options(row)} maxWidth={200} >
            <div className={"items-row"}>{row.is_dir ? '-' : bytesToSize(row.size)}</div>
        </RightMenu>,
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
    {
        name: 'Path',
        grow: 4,
        selector: row => <RightMenu theme="mac" options={options(row)} maxWidth={200} >
            <div className={"items-row"}>{row.abs_path}</div>
        </RightMenu>,
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
];


function open_file_location(row) {
    invoke('my_custom_command', {
        number: 1,
        kw: row.abs_path
    })
}

function Items({items, kw}) {

    function handleClick(row) {
        open_file_location(row);
    }

    return <DataTable
        dense
        columns={columns}
        onRowDoubleClicked={(row) => handleClick(row)}
        data={items}
        customStyles={customStyles}
        highlightOnHover
        pointerOnHover
    />
}


export default Items
