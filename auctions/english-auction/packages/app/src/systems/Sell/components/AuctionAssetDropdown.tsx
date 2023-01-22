import { Dropdown, Button, Icon } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import type { ReactNode } from "react";
import { useEffect, useState } from "react";

import { getTokenText, getAssetText } from "../../Core/utils";

interface AuctionAssetDropdownProps {
  onChange: (isNFT: boolean, assetKey: string) => void;
  assets: CoinQuantity[];
  ariaLabel: string;
}

export const AuctionAssetDropdown = ({
  onChange,
  assets,
  ariaLabel,
}: AuctionAssetDropdownProps) => {
  const [assetIcon, setAssetIcon] = useState("Coin");
  const [assetText, setAssetText] = useState("Token");

  const assetItems: ReactNode = assets?.map((asset: CoinQuantity) => {
    // TODO dynamically load token images and symbols
    // either from some config file or from the wallet
    const iconText = "Coin";
    const text = getTokenText(asset.assetId);
    return (
      <Dropdown.MenuItem key={asset.assetId} textValue={text}>
        <Icon icon={iconText} />
        {text}
      </Dropdown.MenuItem>
    );
  });

  // Set the initial asset text and icon, and asset id
  useEffect(() => {
    const hasInitialAsset = assets.length !== 0;
    const text = getAssetText(!hasInitialAsset);
    const iconText = getAssetIconText();
    setAssetText(text);
    setAssetIcon(iconText);
  }, [assets]);

  const handleTokenTypeSelection = (newTokenType: string) => {
    const isNFT = isTokenTypeNFT(newTokenType);
    const iconText = getAssetIconText(isNFT);
    setAssetIcon(iconText);
    // We don't pass the asset id of the nft because we do not know it from the dropdown
    if (isNFT) {
      onChange(isNFT, "");
    } else {
      onChange(isNFT, newTokenType);
    }
    const text = getAssetText(isNFT);
    setAssetText(text);
  };

  const isTokenTypeNFT = (tokenType: string) => {
    return tokenType === "nft";
  };

  const getAssetIconText = (isNFT: boolean = false) => {
    if (isNFT || !assets || (!!assets && assets.length === 0)) {
      return "Image";
    }
    return "Coin";
  };

  return (
    <Dropdown>
      <Dropdown.Trigger>
        <Button aria-label={ariaLabel} leftIcon={assetIcon}>
          {assetText}
        </Button>
      </Dropdown.Trigger>
      <Dropdown.Menu
        autoFocus
        onAction={(e) => handleTokenTypeSelection(e.toString())}
      >
        {assetItems}
        <Dropdown.MenuItem key="nft" textValue="NFT">
          <Icon icon="Image" />
          NFT
        </Dropdown.MenuItem>
      </Dropdown.Menu>
    </Dropdown>
  );
};
