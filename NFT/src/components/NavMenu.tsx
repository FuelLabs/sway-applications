import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import TableRowsIcon from "@mui/icons-material/TableRows";

import { IconButton } from "@mui/material";
import { useState } from "react";
import { Link } from "./Link";
import { NFTRoutes } from "@/routes";
import { ConnectButton } from "./ConnectButton";

export const NavMenu = () => {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  return (
    <>
      <IconButton
        id="basic-button"
        aria-controls={open ? "basic-menu" : undefined}
        aria-haspopup="true"
        aria-expanded={open ? "true" : undefined}
        onClick={handleClick}
      >
        <TableRowsIcon className="rounded-sm active:bg-gray-600 text-gray-400" />
      </IconButton>
      {/** We need to specify next as the container to get tailwind css to work for material ui */}
      <Menu
        container={() => document.getElementById("__next")}
        id="basic-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        MenuListProps={{ "aria-labelledby": "basic-button" }}
        slotProps={{ paper: { className: "bg-gray-800" } }}
      >
        <MenuItem>
          <Link href={NFTRoutes.explore}>Explore</Link>
        </MenuItem>
        <MenuItem>
          <Link href={NFTRoutes.create}>Create</Link>
        </MenuItem>
        <MenuItem>
          <Link href={NFTRoutes.collection}>Collection</Link>
        </MenuItem>
        <MenuItem>
          <Link href={NFTRoutes.faucet}>Faucet</Link>
        </MenuItem>
        <MenuItem>
          <ConnectButton />
        </MenuItem>
      </Menu>
    </>
  );
};
