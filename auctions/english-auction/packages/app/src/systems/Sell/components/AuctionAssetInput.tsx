import { cssObj } from "@fuel-ui/css";
import { Button, Dropdown, Icon, Input } from "@fuel-ui/react";
import { DECIMAL_UNITS, NativeAssetId } from "fuels";
import type { CoinQuantity } from "fuels";
import { useState } from "react";

import { useAssets } from "~/systems/Core/hooks/useAssets";

// TODO
// Make component look nicer
// add max button to token input for auction asset input

type AuctionAssetInputProps = {
  placeholderAssetId: string;
  placeholderTokenId: string;
  placeholderTokenAmount: string;
  onChange: (id: string, val: string) => void;
  assetIdValue?: string;
  tokenIdValue?: string;
  assetAmountValue?: string;
  id: string;
};

export const AuctionAssetInput = ({
  placeholderAssetId,
  placeholderTokenId,
  placeholderTokenAmount,
  onChange,
  assetIdValue,
  tokenIdValue,
  assetAmountValue,
  id,
}: AuctionAssetInputProps) => {
  const [isNFT, setIsNFT] = useState(false);
  const assets: CoinQuantity[] = useAssets();
  const assetItems = assets?.map((asset: CoinQuantity) => {
    // TODO dynamically load token images and symbols
    // either from some config file or from the wallet
    const iconText = "Coin";
    const text = asset.assetId === NativeAssetId ? "ETH" : "Token";
    return (
      <Dropdown.MenuItem key={asset.assetId} textValue={text}>
        <Icon icon={iconText} />
        {text}
      </Dropdown.MenuItem>
    );
  });

  const handleTokenTypeSelection = (newTokenType: string) => {
    onChange(`tokenType${id}`, newTokenType);
    setIsNFT(newTokenType === "nft");
  };

  return (
    <>
      <Input css={styles.input}>
        {isNFT ? (
          <Input.Number
            id={`tokenId${id}`}
            allowNegative={false}
            autoComplete="off"
            inputMode="numeric"
            onChange={(e) => onChange(`tokenId${id}`, e.target.value)}
            placeholder={placeholderTokenId}
            value={tokenIdValue}
          />
        ) : (
          <Input.Number
            id={`assetAmount${id}`}
            allowedDecimalSeparators={[".", ","]}
            allowNegative={false}
            autoComplete="off"
            inputMode="decimal"
            decimalScale={DECIMAL_UNITS}
            onChange={(e) => onChange(`assetAmount${id}`, e.target.value)}
            placeholder={placeholderTokenAmount}
            thousandSeparator={false}
            value={assetAmountValue}
          />
        )}
      </Input>
      {isNFT && (
        <Input css={styles.input}>
          <Input.Field
            id={`assetId${id}`}
            onChange={(e) => onChange(`assetId${id}`, e.target.value)}
            placeholder={placeholderAssetId}
            value={assetIdValue}
          />
        </Input>
      )}
      <Dropdown>
        <Dropdown.Trigger>
          <Button>Choose Asset Type</Button>
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
    </>
  );
};

const styles = {
  input: cssObj({
    alignSelf: "stretch",
  }),
};
