import React from 'react';
import {
    ContextualMenu, Dropdown,
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
import {useTranslation} from "react-i18next";


const Tab = ({setSelectedKey, selectedKey, kw, setItems}) => {
    const [isOpen, { setTrue: openPanel, setFalse: dismissPanel }] = useBoolean(false);
    const { t } = useTranslation();


    const options = [
        { key: 'fruitsHeader', text: 'EN'},
        { key: 'apple', text: '中文' },

    ];

    const dropdownStyles = {
        dropdown: { width: 300 },
    };
    return (
        <div className={"tabs"}>
            <Panel
                isLightDismiss
                headerText={t("setting-header")}
                isOpen={isOpen}
                onDismiss={dismissPanel}
                // You MUST provide this prop! Otherwise screen readers will just say "button" with no label.
                closeButtonAriaLabel="Close"
            >
                <div className="setting-item">
                    <Dropdown
                        placeholder="Select an option"
                        label="Basic uncontrolled example"
                        options={options}
                        styles={dropdownStyles}
                    />
                </div>
              <div className="setting-item">
                  <TextField label={t("exclude-path-label")} />
                  <div className="added">
                      <Label>I'm a Label</Label>
                      <IconButton iconProps={{iconName: 'RemoveFilter'}}  title="Setting"/>
                  </div>
                  <div className="add">
                      <PrimaryButton text={t("add")} onClick={undefined}  />
                      <PrimaryButton text={t("add")} onClick={undefined}  />
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
                <PivotItem headerText={t("all")} itemIcon="ViewAll2" itemKey="0">
                </PivotItem>
                <PivotItem headerText={t("folder")} itemIcon="FabricFolder" itemKey="1">
                </PivotItem>
                <PivotItem headerText={t("document")} itemIcon="Document" itemKey="2">
                </PivotItem>
                <PivotItem headerText={t("video")} itemIcon="Video" itemKey="3">
                </PivotItem>
                <PivotItem headerText={t("photo")} itemIcon="Photo2" itemKey="4">
                </PivotItem>
            </Pivot>
            <div className={"menu"} onClick={openPanel}>
                <IconButton iconProps={{iconName: 'CollapseMenu'}}  title={t("setting-header")}/>
            </div>
        </div>
    );
};

export default Tab;