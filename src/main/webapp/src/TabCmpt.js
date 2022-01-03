import {Pane, Tab, Tablist} from 'evergreen-ui'
import React from 'react';
import TableCmpt from "./TableCmpt";

function TabCmpt() {
    const [selectedIndex, setSelectedIndex] = React.useState(0)
    const [tabs] = React.useState(['All', 'Video', 'Music'])
    return (
        <Pane height={120}>
            <Tablist marginBottom={16} flexBasis={240} marginRight={24}>
                {tabs.map((tab, index) => (
                    <Tab
                        key={tab}
                        id={tab}
                        onSelect={() => setSelectedIndex(index)}
                        isSelected={index === selectedIndex}
                        aria-controls={`panel-${tab}`}
                    >
                        {tab}
                    </Tab>
                ))}
            </Tablist>
            <Pane padding={16} background="tint1" flex="1" >
                {tabs.map((tab, index) => (
                    <Pane
                        key={tab}
                        id={`panel-${tab}`}
                        role="tabpanel"
                        aria-labelledby={tab}
                        aria-hidden={index !== selectedIndex}
                        display={index === selectedIndex ? 'block' : 'none'}
                    >
                        {/*<Paragraph>Panel {ftab}</Paragraph>*/}
                            <TableCmpt></TableCmpt>

                    </Pane>
                ))}
            </Pane>
        </Pane>
    )
}

export default TabCmpt;
