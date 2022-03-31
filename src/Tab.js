import React from 'react';
import {
    ContextualMenu,
    FontWeights,
    getTheme,
    IconButton, Label,
    mergeStyleSets,
    Modal, Panel,
    Pivot,
    PivotItem, PrimaryButton, TextField
} from "@fluentui/react";
import {search} from "./utils";
import {useBoolean, useId} from "@fluentui/react-hooks";


const Tab = ({setSelectedKey, selectedKey, kw, setItems}) => {
    const [isOpen, { setTrue: openPanel, setFalse: dismissPanel }] = useBoolean(false);


    return (
        <div className={"tabs"}>
            <Panel
                isLightDismiss
                headerText="Setting"
                isOpen={isOpen}
                onDismiss={dismissPanel}
                // You MUST provide this prop! Otherwise screen readers will just say "button" with no label.
                closeButtonAriaLabel="Close"
            >
              <div className="setting-item">
                  <TextField label="Exclude Path" />
                  <div className="added">
                      <Label>I'm a Label</Label>
                      <IconButton iconProps={{iconName: 'RemoveFilter'}}  title="Setting"/>
                  </div>
                  <div className="add">
                      <PrimaryButton text="Add" onClick={undefined}  />
                  </div>
              </div>
            </Panel>

            <Pivot aria-label="Count and Icon Pivot Example" selectedKey={String(selectedKey)} onLinkClick={(event) => {
                let key = event.key.substr(1);
                setSelectedKey(key)
                search(kw, key).then(value => {
                    setItems(value)
                })
            }}>
                {/*https://uifabricicons.azurewebsites.net/?help*/}
                <PivotItem headerText="All" itemIcon="ViewAll2" itemKey="0">
                </PivotItem>
                <PivotItem headerText="Folder" itemIcon="FabricFolder" itemKey="1">
                </PivotItem>
                <PivotItem headerText="Document" itemIcon="Document" itemKey="2">
                </PivotItem>
                <PivotItem headerText="Video" itemIcon="Video" itemKey="3">
                </PivotItem>
                <PivotItem headerText="Photo" itemIcon="Photo2" itemKey="4">
                </PivotItem>
            </Pivot>
            <div className={"menu"} onClick={openPanel}>
                <IconButton iconProps={{iconName: 'CollapseMenu'}}  title="Setting"/>
            </div>
        </div>
    );
};

export default Tab;