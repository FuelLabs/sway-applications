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
};

export const AuctionAssetInput = ({
  placeholderAssetId,
  placeholderTokenId,
  placeholderTokenAmount,
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
    setIsNFT(newTokenType === "nft");
  };

  return (
    <>
      <Input css={styles.input}>
        {isNFT ? (
          <Input.Number
            allowNegative={false}
            autoComplete="off"
            inputMode="numeric"
            placeholder={placeholderTokenId}
          />
        ) : (
          <Input.Number
            allowedDecimalSeparators={[".", ","]}
            allowNegative={false}
            autoComplete="off"
            inputMode="decimal"
            decimalScale={DECIMAL_UNITS}
            placeholder={placeholderTokenAmount}
            thousandSeparator={false}
          />
        )}
      </Input>
      {isNFT && (
        <Input css={styles.input}>
          <Input.Field placeholder={placeholderAssetId} />
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
          {/* TODO figure out how to remove this extra needed {} */}
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
