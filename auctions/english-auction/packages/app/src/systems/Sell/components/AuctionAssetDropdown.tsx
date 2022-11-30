import { Dropdown, Button, Icon } from "@fuel-ui/react";
import { CoinQuantityLike, NativeAssetId } from "fuels";
import type { CoinQuantity } from "fuels";
import { useEffect, useState } from "react";
import { useAssets } from "~/systems/Core/hooks/useAssets";

interface AuctionAssetDropdownProps {
  onChange: (isNFT: boolean, assetKey: string) => void;
  assets: CoinQuantity[]
}

export const AuctionAssetDropdown = ({
  onChange,
  assets,
}: AuctionAssetDropdownProps) => {
  const [assetIcon, setAssetIcon] = useState("Coin");
  const [assetText, setAssetText] = useState("Token");

  //const assets = useAssets();

  const getTokenText = (assetId: string) => {
    return assetId === NativeAssetId ? "ETH" : "Token";
  };

  const assetItems = assets?.map((asset: CoinQuantity) => {
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

  // Set the initial asset text
  useEffect(() => {
    const text = getAssetText();
    // if (!!assets) {
    //   onChange(false, assets[0].assetId);
    // }
    setAssetText(text);
  }, [assets]);

  const handleTokenTypeSelection = (newTokenType: string) => {
    const iconText = newTokenType === "nft" ? "Image" : "Coin";
    setAssetIcon(iconText);

    const isNFT = isTokenTypeNFT(newTokenType);
    // Pass to parent component
    onChange(isNFT, newTokenType);
    const text = getAssetText(isNFT);
    setAssetText(text);
  };

  const isTokenTypeNFT = (tokenType: string) => {
    return tokenType === "nft";
  };

  const getAssetText = (isNFT: boolean = false) => {
    if (isNFT || !assets) {
      return "NFT";
    }
    const text = getTokenText(assets[0].assetId);
    return text;
  };

  return (
    <Dropdown>
      <Dropdown.Trigger>
        <Button leftIcon={assetIcon}>{assetText}</Button>
      </Dropdown.Trigger>
      <Dropdown.Menu
        autoFocus
        onAction={(e) => handleTokenTypeSelection(e.toString())}
      >
        {assetItems ? assetItems : {}}
        <Dropdown.MenuItem key="nft" textValue="NFT">
          <Icon icon="Image" />
          NFT
        </Dropdown.MenuItem>
      </Dropdown.Menu>
    </Dropdown>
  );
};
