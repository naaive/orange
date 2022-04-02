import React, {useEffect, useState} from 'react';
import {
    ContextualMenu,
    DefaultButton,
    Dialog,
    DialogFooter,
    DialogType,
    Dropdown,
    IconButton,
    Label,
    MessageBar,
    MessageBarType,
    Panel,
    Pivot,
    PivotItem,
    PrimaryButton,
    TextField
} from "@fluentui/react";
import {add_exclude_path, change_lang, get_exclude_paths, reindex, remove_exclude_path, search} from "./utils";
import {useBoolean, useId} from "@fluentui/react-hooks";
import {useTranslation} from "react-i18next";
import i18next from "i18next";

const dialogStyles = {main: {maxWidth: 450}};
const dragOptions = {
    moveMenuItemText: 'Move',
    closeMenuItemText: 'Close',
    menu: ContextualMenu,
    keepInBounds: true,
};


const Tab = ({setSelectedKey, selectedKey, kw, setItems}) => {

    const [isOpen, {setTrue: openPanel, setFalse: dismissPanel}] = useBoolean(false);
    const {t} = useTranslation();
    let [excludePath, setExcludePath] = useState();
    let [show, setShow] = useState(false);
    let [toastText, setToastText] = useState('');
    let [excludePaths, setExcludePaths] = useState([]);

    const [hideDialog, {toggle: toggleHideDialog}] = useBoolean(true);
    const [isDraggable, {toggle: toggleIsDraggable}] = useBoolean(false);
    const labelId = useId('dialogLabel');
    const subTextId = useId('subTextLabel');

    const dialogContentProps = {
        type: DialogType.normal,
        title: t('reindex'),
        closeButtonAriaLabel: 'Close',
        subText: t("reindex-dialog"),
    };

    function toast(txt) {
        setToastText(txt);
        setShow(true);
        setTimeout(() => setShow(false), 2000);
    }

    useEffect(async () => {
        setExcludePaths(await get_exclude_paths())
    }, [excludePaths])
    const options = [
        {key: 'en', text: 'EN'},
        {key: 'zh-CN', text: '中文'},
    ];

    const dropdownStyles = {
        dropdown: {width: 300},
    };

    async function handleAddExcludePath() {
        if (await add_exclude_path(excludePath) === 1) {
            toast(t("add_exclude_path_err"));

        } else {
            setExcludePath("");
            setExcludePaths(await get_exclude_paths());
        }
    }

    async function handleRemoveExcludePath(path) {
        await remove_exclude_path(path)
        setExcludePaths(await get_exclude_paths());
    }

    function handle_reindex() {
        toggleHideDialog();
        reindex();
    }

    function handle_lang_change(_, item) {
        let key = item.key;
        setSelectedKey(key);
        change_lang(key);
        i18next.changeLanguage(key, (err, t) => {
            if (err) return console.log('something went wrong loading', err);
            t('key'); // -> same as i18next.t
        });
    }

    const modalProps = React.useMemo(
        () => ({
            titleAriaId: labelId,
            subtitleAriaId: subTextId,
            isBlocking: false,
            styles: dialogStyles,
            dragOptions: isDraggable ? dragOptions : undefined,
        }),
        [isDraggable, labelId, subTextId],
    );

    return (
        <div className={"tabs"}>

            <Dialog
                hidden={hideDialog}
                onDismiss={toggleHideDialog}
                dialogContentProps={dialogContentProps}
                modalProps={modalProps}
            >
                <DialogFooter>
                    <PrimaryButton onClick={handle_reindex} text={t("confirm")}/>
                    <DefaultButton onClick={toggleHideDialog} text={t("cancel")}/>
                </DialogFooter>
            </Dialog>
            <Panel
                isLightDismiss
                headerText={t("setting-header")}
                isOpen={isOpen}
                onDismiss={dismissPanel}
                // You MUST provide this prop! Otherwise screen readers will just say "button" with no label.
                closeButtonAriaLabel="Close"
            >
                {
                    show ? <MessageBar messageBarType={MessageBarType.error}>
                        {toastText}
                    </MessageBar> : ""
                }
                <div className="setting-item">
                    <Dropdown
                        onChange={handle_lang_change}
                        label={t("lang")}
                        selectedKey={selectedKey}
                        options={options}
                        styles={dropdownStyles}
                    />
                </div>
                <div className="setting-item">

                    <TextField label={t("exclude-path-label")} value={excludePath} placeholder={t("path")}
                               onChange={(e) => setExcludePath(e.target.value)}/>
                    <div className="added">
                        {
                            excludePaths.map(x => <div className="added-item">
                                <Label>{x}</Label>
                                <IconButton iconProps={{iconName: 'RemoveFilter'}} title={t("remove")}
                                            onClick={() => handleRemoveExcludePath(x)}/>
                            </div>)
                        }

                    </div>
                    <div className="add">
                        <DefaultButton text={t("add")} onClick={handleAddExcludePath}/>
                        <DefaultButton text={t("reindex")} onClick={toggleHideDialog}/>
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
                <IconButton iconProps={{iconName: 'CollapseMenu'}} title={t("setting-header")}/>
            </div>
        </div>
    );
};

export default Tab;