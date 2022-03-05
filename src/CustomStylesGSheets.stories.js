import React from 'react';
// import Icon from '@material-ui/icons/Apps';
import DataTable from 'react-data-table-component';

import CustomMaterialMenu from './shared/CustomMaterialMenu';
import moment from "moment";
import * as R from "ramda";
import Folder from "./folder.svg";
import {defaultStyles, FileIcon} from "react-file-icon";

const data = [
    {
        id: 1,
        title: 'Cutting Costs',
        by: 'me',
        lastOpened: 'Aug 7 9:52 AM',
    },
    {
        id: 2,
        title: 'Wedding Planner',
        by: 'me',
        lastOpened: 'Sept 14 2:52 PM',
    },
    {
        id: 3,
        title: 'Expense Tracker',
        by: 'me',
        lastOpened: 'Sept 12 2:41 PM',
    },

];

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
            backgroundColor: '#edf2f7',
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
    // {
    // 	cell: () => <Icon style={{ fill: '#43a047' }} />,
    // 	width: '56px', // custom width for icon button
    // 	style: {
    // 		borderBottom: '1px solid #FFFFFF',
    // 		marginBottom: '-1px',
    // 	},
    // },
    {
        name: 'Name',
        selector: row => {
            let isDir = R.prop('is_dir')(row);
            let name = R.prop("name")(row);
            const extSplit = R.split('.');
            let ext = R.last(extSplit(name));

            let icon = isDir ? <img src={Folder}/> :
                <FileIcon extension={ext} {...defaultStyles[ext]} />;
            return <>
               <div className="icon">
                    <span  className={"img"}>
                    {icon}
                </span>
                   <span>
                    {row.name}
                </span>
               </div>

            </>;
        },
        width: '360px',
        style: {
            color: '#202124',
            fontSize: '14px',
            fontWeight: 500,
        },
    },
    {
        name: 'Size',
        width: '80px',

        selector: row => bytesToSize(row.size),
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
    {
        name: 'Last Modified',
        width: '160px',
        selector: row => moment(R.prop('mod_at', row.mod_at)).format("YYYY-MM-DD h:mm:ss"),
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
    {
        name: 'Path',
        width: '260px',
        selector: row => row.abs_path,
        style: {
            color: 'rgba(0,0,0,.54)',
        },
    },
    {
        cell: row => <CustomMaterialMenu size="small" row={row}/>,
        allowOverflow: true,
        button: true,
        width: '56px',
    },
];


function GoogleSheetsEsque({items, kw}) {

    console.log(items)
    return <DataTable
        columns={columns}
        data={items}
        customStyles={customStyles}
        highlightOnHover
        pointerOnHover
    />
}


export default GoogleSheetsEsque
