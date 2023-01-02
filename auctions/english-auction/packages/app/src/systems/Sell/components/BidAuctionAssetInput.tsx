import { Form } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import { useState } from "react";

import { DropdownContainer } from "./DropdownContainer";
import { IdentityFormInput } from "./IdentityFormInput";

interface BidAuctionAssetInputProps {
  nftAssetIdValue: string;
  onChange: (id: string, val: string) => void;
  assets: CoinQuantity[];
}

export const BidAuctionAssetInput = ({
  onChange,
  nftAssetIdValue,
  assets,
}: BidAuctionAssetInputProps) => {
  const [isNFT, setIsNFT] = useState(false);

  const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
    setIsNFT(newIsNFT);
    if (!newIsNFT) {
      onChange("assetIdBid", assetType);
    }
  };

  return (
    <DropdownContainer onChange={handleAssetChange} assets={assets}>
      <Form.Control isRequired>
        <Form.Label>Bid Asset</Form.Label>
        {isNFT && (
          <IdentityFormInput
            onChange={onChange}
            label="Bid NFT Asset Id"
            objKey="nftAssetIdBid"
            identityValue={nftAssetIdValue}
          />
        )}
      </Form.Control>
    </DropdownContainer>
  );
};
