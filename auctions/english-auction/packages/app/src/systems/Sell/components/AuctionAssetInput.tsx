import { cssObj } from "@fuel-ui/css";
import { Button, Dropdown, Icon, Input, Form, Flex } from "@fuel-ui/react";
import { DECIMAL_UNITS, NativeAssetId } from "fuels";
import type { CoinQuantity } from "fuels";
import { useState } from "react";

import { useAssets } from "~/systems/Core/hooks/useAssets";
import { AuctionAssetDropdown } from "./AuctionAssetDropdown";

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

  return (
    <Flex>
      <Flex grow="2">
        <Form.Control isRequired css={{ minWidth: "100%"}}>
          <Input>
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
        </Form.Control>
      </Flex>
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
      <Flex grow={0}>
        <AuctionAssetDropdown onChange={(e) => setIsNFT(e)} />
      </Flex>
    </Flex>
  );
};

const styles = {
  input: cssObj({
    alignSelf: "stretch",
  }),
};
