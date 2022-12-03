import { cssObj } from "@fuel-ui/css";
import { Input, Form, Flex } from "@fuel-ui/react";
import { DECIMAL_UNITS } from "fuels";
import type { CoinQuantity } from "fuels";
import { useState } from "react";

import { DropdownContainer } from "./DropdownContainer";
import { NFTAssetIdInput } from "./NFTAssetIdInput";

// TODO
// Make component look nicer
// add max button to token input for auction asset input
// or show current balance of specified asset

type SellAuctionAssetInputProps = {
  nftContractIdFormLabel: string;
  nftIdFormLabel: string;
  tokenAmountLabel: string;
  onChange: (id: string, val: string) => void;
  nftAssetIdValue?: string;
  nftTokenIdValue?: string;
  assetAmountValue?: string;
  assets: CoinQuantity[];
};

export const SellAuctionAssetInput = ({
  nftContractIdFormLabel,
  nftIdFormLabel,
  tokenAmountLabel,
  onChange,
  nftAssetIdValue,
  nftTokenIdValue,
  assetAmountValue,
  assets,
}: SellAuctionAssetInputProps) => {
  const [isNFT, setIsNFT] = useState(false);
  const id = "nftAssetIdSell";

  const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
    setIsNFT(newIsNFT);
    if (newIsNFT) {
      onChange(id, assetType);
    } else {
      onChange("assetIdSell", assetType);
    }
  }

  return (
    <DropdownContainer onChange={handleAssetChange} assets={assets}>
      {isNFT ? (
        <Flex direction="column" css={{ minWidth: "100%" }}>
          <Form.Control isRequired css={{ minWidth: "100%" }}>
            <Form.Label>{nftIdFormLabel}</Form.Label>
            <Input>
              <Input.Number
                id='tokenIdSell'
                allowNegative={false}
                autoComplete="off"
                inputMode="numeric"
                onChange={(e) => onChange('tokenIdSell', e.target.value)}
                placeholder="0"
                value={nftTokenIdValue}
              />
            </Input>
          </Form.Control>
          <NFTAssetIdInput onChange={onChange} label="Sell NFT Asset Id" id={id} nftAssetIdValue={nftAssetIdValue!} />
        </Flex>
      ) : (
        <Form.Control isRequired css={{ minWidth: "100%" }}>
          <Form.Label>{tokenAmountLabel}</Form.Label>
          <Input>
            <Input.Number
              id='assetAmountSell'
              allowedDecimalSeparators={[".", ","]}
              allowNegative={false}
              autoComplete="off"
              inputMode="decimal"
              decimalScale={DECIMAL_UNITS}
              onChange={(e) => onChange('assetAmountSell', e.target.value)}
              placeholder="0.0"
              thousandSeparator={false}
              value={assetAmountValue}
            />
          </Input>
        </Form.Control>
      )}
    </DropdownContainer>
  );
};

const styles = {
  input: cssObj({
    alignSelf: "stretch",
  }),
};
