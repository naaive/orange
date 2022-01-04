import {Pane, Tab, Tablist} from 'evergreen-ui'
import React from 'react';
import TableCmpt from "./TableCmpt";
const R = require('ramda');

function TabCmpt({items}) {
    const [selectedIndex, setSelectedIndex] = React.useState(0)
    const [tabs] = React.useState(['All', 'Video', 'Music'])
    return (
        <Pane >
            {/*<Tablist marginBottom={16} flexBasis={240} marginRight={24}>*/}
            {/*    {tabs.map((tab, index) => (*/}
            {/*        <Tab*/}
            {/*            key={tab}*/}
            {/*            id={tab}*/}
            {/*            onSelect={() => setSelectedIndex(index)}*/}
            {/*            isSelected={index === selectedIndex}*/}
            {/*            aria-controls={`panel-${tab}`}*/}
            {/*        >*/}
            {/*            {tab}*/}
            {/*        </Tab>*/}
            {/*    ))}*/}
            {/*</Tablist>*/}
            <TableCmpt items={items}/>
        </Pane>
    )
}

export default TabCmpt;
