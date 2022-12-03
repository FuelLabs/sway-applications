import { cssObj } from "@fuel-ui/css";
import { Input, Form, Flex } from "@fuel-ui/react";
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
  nftContractIdFormLabel: string;
  nftIdFormLabel: string;
  onChange: (id: string, val: string) => void;
  nftAssetIdValue?: string;
  nftTokenIdValue?: string;
  assetAmountValue?: string;
  assets: CoinQuantity[];
};

export const SellAuctionAssetInput = ({
  nftContractIdFormLabel,
  nftIdFormLabel,
  onChange,
  nftAssetIdValue,
  nftTokenIdValue,
  assetAmountValue,
  assets,
}: SellAuctionAssetInputProps) => {
  const [isNFT, setIsNFT] = useState(false);
  const key = "nftAssetIdSell";

  const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
    setIsNFT(newIsNFT);
    if (newIsNFT) {
      onChange(key, assetType);
    } else {
      onChange("assetIdSell", assetType);
    }
  }

  // TODO refactor: change outer flex to stack
  return (
    <DropdownContainer onChange={handleAssetChange} assets={assets}>
      {isNFT ? (
        <Flex direction="column" css={{ minWidth: "100%" }}>
          <NumericFormInput
            onChange={(e) => onChange('tokenIdSell', e)}
            formLabel="Sell NFT Id"
            formValue={nftTokenIdValue!}
            key="tokenIdSell"
          />
          <NFTAssetIdInput
            onChange={onChange}
            label="Sell NFT Asset Id"
            key={key}
          nftAssetIdValue={nftAssetIdValue!} />
        </Flex>
      ) : (
        <AssetAmountInput
          onChange={onChange}
          key="assetAmountSell"
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
