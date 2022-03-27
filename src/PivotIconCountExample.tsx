import * as React from 'react';
import {Icon, IStyleSet, Label, ILabelStyles, Pivot, IPivotItemProps, PivotItem, TextField} from '@fluentui/react';

const labelStyles: Partial<IStyleSet<ILabelStyles>> = {
    root: { marginTop: 10 },
};

export const PivotIconCountExample: React.FunctionComponent = () => {
    return (
        <div>
            <Pivot aria-label="Count and Icon Pivot Example">
                <PivotItem headerText="All " itemIcon="CompaRacingssNW">
                </PivotItem>
                <PivotItem headerText="Doc"  itemIcon="Document">
                </PivotItem>
                <PivotItem headerText="Video" itemIcon="Video">
                </PivotItem>
                <PivotItem headerText="Audio" itemIcon="MusicInCollectionFill">
                </PivotItem>


            </Pivot>
        </div>
    );
};

function _customRenderer(
    link?: IPivotItemProps,
    defaultRenderer?: (link?: IPivotItemProps) => JSX.Element | null,
): JSX.Element | null {
    if (!link || !defaultRenderer) {
        return null;
    }

    return (
        <span style={{ flex: '0 1 100%' }}>
      {defaultRenderer({ ...link, itemIcon: undefined })}
            <Icon iconName={link.itemIcon} style={{ color: 'red' }} />
    </span>
    );
}
