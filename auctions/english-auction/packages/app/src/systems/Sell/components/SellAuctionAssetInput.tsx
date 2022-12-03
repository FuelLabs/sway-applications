import { cssObj } from "@fuel-ui/css";
import { Input, Form, Flex, Stack } from "@fuel-ui/react";
import { DECIMAL_UNITS } from "fuels";
import type { CoinQuantity } from "fuels";
import { useState } from "react";

import { DropdownContainer } from "./DropdownContainer";
import { NFTAssetIdInput } from "./NFTAssetIdInput";
import { AssetAmountInput } from "./AssetAmountInput";
import { NumericFormInput } from "./NumericFormInput";

// TODO
// Make component look nicer
// add max button to token input for auction asset input
// or show current balance of specified asset

type SellAuctionAssetInputProps = {
  onChange: (id: string, val: string) => void;
  nftAssetIdValue?: string;
  nftTokenIdValue?: string;
  assetAmountValue?: string;
  assets: CoinQuantity[];
};

export const SellAuctionAssetInput = ({
  onChange,
  nftAssetIdValue,
  nftTokenIdValue,
  assetAmountValue,
  assets,
}: SellAuctionAssetInputProps) => {
  const [isNFT, setIsNFT] = useState(false);

  const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
    setIsNFT(newIsNFT);
    if (!newIsNFT) {
      onChange("assetIdSell", assetType);
    }
  }

  // TODO refactor: change outer flex to stack
  return (
    <DropdownContainer onChange={handleAssetChange} assets={assets}>
      {isNFT ? (
        <Stack css={{ minWidth: "100%" }}>
          <NumericFormInput
            onChange={onChange}
            formLabel="Sell NFT Id"
            formValue={nftTokenIdValue!}
            objKey="tokenIdSell"
            isRequired={true}
          />
          <NFTAssetIdInput
            onChange={onChange}
            label="Sell NFT Asset Id"
            objKey="nftAssetIdSell"
            nftAssetIdValue={nftAssetIdValue!} />
        </Stack>
      ) : (
        <AssetAmountInput
          onChange={onChange}
          objKey="assetAmountSell"
          assetAmountLabel="Sell Asset Amount"
          assetAmountValue={assetAmountValue!}
        />
      )}
    </DropdownContainer>
  );
};

const styles = {
  input: cssObj({
    alignSelf: "stretch",
  }),
};
