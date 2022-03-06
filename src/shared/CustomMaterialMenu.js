import React from 'react';
import IconButton from '@material-ui/core/IconButton';
import Menu from '@material-ui/core/Menu';
import MenuItem from '@material-ui/core/MenuItem';
import MoreVertIcon from '@material-ui/icons/MoreVert';

// eslint-disable-next-line react/prop-types
export default ({ row, onDeleteRow, size }) => {
	const [anchorEl, setAnchorEl] = React.useState(null);

	const handleClick = event => {
		setAnchorEl(event.currentTarget);
	};

	const handleClose = () => {
		setAnchorEl(null);
	};
	const deleteRow = () => {
		if (onDeleteRow) {
			onDeleteRow(row);
		}
	};

	return (
		<div>
			<IconButton aria-label="more" aria-controls="long-menu" aria-haspopup="true" onClick={handleClick} size={size}>
				<MoreVertIcon />
			</IconButton>
			<Menu
				id="menu"
				getContentAnchorEl={null}
				anchorOrigin={{
					vertical: 'bottom',
					horizontal: 'center',
				}}
				transformOrigin={{
					vertical: 'top',
					horizontal: 'center',
				}}
				anchorEl={anchorEl}
				keepMounted
				open={Boolean(anchorEl)}
				onClose={handleClose}
			>
				<MenuItem>Open</MenuItem>
				<MenuItem>Copy Path</MenuItem>


			</Menu>
		</div>
	);
};
