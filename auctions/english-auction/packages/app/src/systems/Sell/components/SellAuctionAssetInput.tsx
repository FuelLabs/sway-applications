import { cssObj } from "@fuel-ui/css";
import { Stack } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import { useState } from "react";

import { AssetAmountInput } from "../../Core/components/AssetAmountInput";
import { DropdownContainer } from "./DropdownContainer";
import { IdentityFormInput } from "./IdentityFormInput";
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
  };

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
          <IdentityFormInput
            onChange={onChange}
            label="Sell NFT Asset Id"
            objKey="nftAssetIdSell"
            identityValue={nftAssetIdValue!}
          />
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
