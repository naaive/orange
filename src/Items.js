import React from 'react';
import DataTable from 'react-data-table-component';

import moment from "moment";
import * as R from "ramda";
import Folder from "./folder.svg";
import {defaultStyles, FileIcon} from "react-file-icon";
import {invoke} from "@tauri-apps/api";


const customStyles = {
    headRow: {
        style: {
            border: 'none',
        },
    },
    headCells: {
        style: {
            color: '#202124',
            fontSize: '14px',
        },
    },
    rows: {
        highlightOnHoverStyle: {
            backgroundColor: '#e8e8e8',
            borderBottomColor: '#FFFFFF',
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
            borderBottom: '1px solid #FFFFFF',
            marginBottom: '-1px',
        },
    },
    {
        name: 'Name',
        selector: row => {
            return row.name;
        },
        style: {
            color: '#202124',
            fontSize: '14px',
            fontWeight: 500,
        },
    },
    {
        name: 'Size',
        maxWidth: '80px',

        selector: row => bytesToSize(row.size),
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
    {
        name: 'Last Modified',
        maxWidth: '160px',
        selector: row => moment(R.prop('mod_at', row.mod_at)).format("YYYY-MM-DD h:mm:ss"),
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
    {
        name: 'Path',
        grow: 3,
        selector: row => {

            return row.abs_path;
        },
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
    // {
    //     cell: row => <CustomMaterialMenu size="small" row={row}/>,
    //     allowOverflow: true,
    //     button: true,
    //     grow: 3,
    //
    // },
];


function Items({items, kw}) {

    function handleClick(row) {
        invoke('my_custom_command', {
            number: 1,
            kw:row.abs_path
        })
    }

    return <DataTable
        columns={columns}
        onRowDoubleClicked={(row) => handleClick(row)}
        data={items}
        customStyles={customStyles}
        highlightOnHover
        pointerOnHover
    />
}


export default Items
