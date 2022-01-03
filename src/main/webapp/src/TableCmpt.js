import {
    CogIcon,
    DuplicateIcon,
    FolderOpenIcon,
    IconButton,
    majorScale,
    Pane,
    Table,
    TickIcon,
    TrashIcon
} from 'evergreen-ui'
import React from "react";
import './TableCmpt.css'

function TableCmpt() {
    return <div className="table">
        <Table.Body>
            {/*<Table.Head>*/}
            {/*    <Table.TextCell flexBasis={560} flexShrink={0} flexGrow={0}>*/}
            {/*        Fixed width*/}
            {/*    </Table.TextCell>*/}
            {/*    <Table.TextCell>Flex me col 2</Table.TextCell>*/}
            {/*    <Table.TextCell>Flex me col 3</Table.TextCell>*/}
            {/*</Table.Head>*/}
            <Table.Body>
                <Table.Row>
                    <Table.TextCell>
                        Fixed width
                    </Table.TextCell>
                    <Table.TextCell>Flex me col 2</Table.TextCell>
                    <Table.TextCell>
                        <Pane display="flex" alignItems="center">
                            <IconButton size={"small"} icon={DuplicateIcon} marginRight={majorScale(2)} />
                            <IconButton size={"small"} icon={FolderOpenIcon} intent="success" />
                        </Pane>
                    </Table.TextCell>
                </Table.Row>
                <Table.Row>
                    <Table.TextCell>
                        Fixed width
                    </Table.TextCell>
                    <Table.TextCell>Flex me col 2</Table.TextCell>
                    <Table.TextCell>
                        <Pane display="flex" alignItems="center">
                            <IconButton size={"small"} icon={DuplicateIcon} marginRight={majorScale(2)} />
                            <IconButton size={"small"} icon={FolderOpenIcon} intent="success" />
                        </Pane>
                    </Table.TextCell>
                </Table.Row>
                <Table.Row>
                    <Table.TextCell>
                        Fixed width
                    </Table.TextCell>
                    <Table.TextCell>Flex me col 2</Table.TextCell>
                    <Table.TextCell>
                        <Pane display="flex" alignItems="center">
                            <IconButton size={"small"} icon={DuplicateIcon} marginRight={majorScale(2)} />
                            <IconButton size={"small"} icon={FolderOpenIcon} intent="success" />
                        </Pane>
                    </Table.TextCell>
                </Table.Row>




            </Table.Body>
        </Table.Body>
    </div>
}

export default TableCmpt;