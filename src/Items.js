import React, {useEffect, useState} from 'react';
import {
    ContextualMenu, DefaultButton,
    DetailsList,
    DetailsListLayoutMode,
    IColumn,
    mergeStyleSets,
    SelectionMode,
    TooltipHost
} from "@fluentui/react";
import {invoke} from "@tauri-apps/api";
import * as R from "ramda";
import {getFileTypeIconProps, FileIconType, initializeFileTypeIcons} from '@fluentui/react-file-type-icons';
import {Icon} from "office-ui-fabric-react";
import moment from "moment";
import copy from "copy-to-clipboard";
import {open_file_location, open_file_location_in_terminal} from "./utils";
import RightMenu from '@right-menu/react'
import {useConst} from "@chakra-ui/react";

initializeFileTypeIcons(undefined);

function bytesToSize(bytes) {
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 Byte';
    const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
    return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i];
}

function tsFmt(mod_at) {
    return moment.unix(mod_at).format("YYYY/MM/DD");
}

function _getKey(item, index) {
    return item.key;
}

const classNames = mergeStyleSets({
    fileIconHeaderIcon: {
        padding: 0,
        fontSize: "12px"
    },
    fileIconCell: {
        textAlign: "center",
        selectors: {
            "&:before": {
                content: ".",
                display: "inline-block",
                verticalAlign: "middle",
                height: "100%",
                width: "0px",
                visibility: "hidden"
            }
        }
    },
    fileIconImg: {
        verticalAlign: "middle",
        maxHeight: "16px",
        maxWidth: "16px"
    },
    controlWrapper: {
        display: "flex",
        flexWrap: "wrap"
    },
    exampleToggle: {
        display: "inline-block",
        marginBottom: "10px",
        marginRight: "30px"
    },
    selectionDetails: {
        marginBottom: "20px"
    }
});

const columns = [
    {
        key: "column1",
        name: "File Type",
        className: classNames.fileIconCell,
        iconClassName: classNames.fileIconHeaderIcon,
        ariaLabel:
            "Column operations for File type, Press to sort on File type",
        iconName: "Page",
        isIconOnly: true,
        fieldName: "name",
        minWidth: 16,
        maxWidth: 16,

        onRender: (item) => {
            let isDir = R.prop('is_dir')(item);
            let name = R.prop("name")(item);
            const extSplit = R.split('.');
            let ext = R.last(extSplit(name));
            // let ext = "exe";
            // item.name
            return (
               <div>
                   <TooltipHost content={`${item.fileType} file`}>
                       {
                           isDir ? <Icon {...getFileTypeIconProps({
                                   type: FileIconType.folder,
                                   size: 20,
                                   imageFileType: 'svg'
                               })} /> :
                               <Icon {...getFileTypeIconProps({extension: ext, size: 20, imageFileType: 'png'})} />
                       }

                   </TooltipHost>
               </div>
            );
        }
    },
    {
        key: "column2",
        name: "Name",
        fieldName: "name",
        minWidth: 50,
        maxWidth: 200,
        isRowHeader: true,
        isResizable: true,
        data: "string",
        isPadded: true
    },
    {
        key: "column3",
        name: "Last Modified",
        fieldName: "mod_at",
        minWidth: 70,
        maxWidth: 90,
        isResizable: true,
        data: "number",
        onRender: (item) => {
            return <span>{tsFmt(item.mod_at)}</span>;
        },
        isPadded: true
    },
    {
        key: "column4",
        name: "Size",
        fieldName: "size",
        minWidth: 40,
        maxWidth: 60,
        isSorted: false,
        isResizable: true,
        isCollapsible: true,
        data: "string",
        onRender: (item) => {
            return <span>{bytesToSize(item.size)}</span>;
        },
        isPadded: true
    },
    {
        key: "column5",
        name: "Path",
        fieldName: "abs_path",
        minWidth: 70,
        isSorted: false,
        isResizable: true,
        isCollapsible: true,
        data: "number",
        onRender: (item) => {
            return <span>{item.abs_path}</span>;
        }
    }
];

function options(row) {

    return [
        {
            type: 'li',
            text: 'Open',
            callback: () => {
                open_file_location(row)
            }
        },
        //
        {
            type: 'li',
            text: 'Copy Path',
            callback: () => copy(row.abs_path)
        },

        {
            type: 'li',
            text: 'Open in Terminal',
            callback: () => {
                open_file_location_in_terminal(row)
            }
        },
    ]
}

function Items({items, setItems}) {

    const menuProps = useConst(() => ({
        shouldFocusOnMount: true,
        shouldFocusOnContainer: true,
        items: [
            {key: 'rename', text: 'Rename', onClick: () => console.log('Rename clicked')},
            {key: 'edit', text: 'Edit', onClick: () => console.log('Edit clicked')},
            {key: 'properties', text: 'Properties', onClick: () => console.log('Properties clicked')},
            {key: 'linkNoTarget', text: 'Link same window', href: 'http://bing.com'},
            {key: 'linkWithTarget', text: 'Link new window', href: 'http://bing.com', target: '_blank'},
            {key: 'disabled', text: 'Disabled item', disabled: true},
        ],
    }));

    return (
        <div>
            <DetailsList
                onRenderRow={(props, Row) => {
                    let row = props.item;
                    return <RightMenu theme="mac" options={options(row)} maxWidth={200} style={{cursor: "pointer"}}>
                        <div onDoubleClick={() => open_file_location(row)}>
                            <Row persistMenu menuProps={menuProps} data-foo="bar" {...props} />
                        </div>
                    </RightMenu>;
                }}

                items={items}
                compact={true}
                columns={columns}
                selectionMode={SelectionMode.none}
                getKey={_getKey}
                setKey="none"
                layoutMode={DetailsListLayoutMode.justified}
                isHeaderVisible={true}
                // onItemInvoked={this._onItemInvoked}
            />


        </div>
    );
}

export default Items;