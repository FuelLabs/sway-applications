import { cssObj } from "@fuel-ui/css";
import { Button, Dropdown, Icon, Input, Form, Flex } from "@fuel-ui/react";
import { DECIMAL_UNITS, NativeAssetId } from "fuels";
import type { CoinQuantity } from "fuels";
import { useState } from "react";

import { useAssets } from "~/systems/Core/hooks/useAssets";
import { AuctionAssetDropdown } from "./AuctionAssetDropdown";
import { DropdownContainer } from "./DropDownContainer";

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

  const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
    setIsNFT(newIsNFT);
    if (newIsNFT) {
      onChange("nftAssetIdSell", assetType);
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
          <Form.Control isRequired css={{ minWidth: "100%" }}>
            <Form.Label>{nftContractIdFormLabel}</Form.Label>
            <Input css={styles.input}>
              <Input.Field
                id='nftAssetIdSell'
                onChange={(e) => onChange('nftAssetIdSell', e.target.value)}
                placeholder="0x000...000"
                value={nftAssetIdValue}
              />
            </Input>
          </Form.Control>
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

  // return (
  //   <Flex>
  //     <Flex grow="2">
  //       {isNFT ? (
  //         <Flex direction="column" css={{ minWidth: "100%" }}>
  //           <Form.Control isRequired css={{ minWidth: "100%" }}>
  //             <Form.Label>{nftIdFormLabel}</Form.Label>
  //             <Input>
  //               <Input.Number
  //                 id='tokenIdSell'
  //                 allowNegative={false}
  //                 autoComplete="off"
  //                 inputMode="numeric"
  //                 onChange={(e) => onChange('tokenIdSell', e.target.value)}
  //                 placeholder="0"
  //                 value={nftTokenIdValue}
  //               />
  //             </Input>
  //           </Form.Control>
  //           <Form.Control isRequired css={{ minWidth: "100%" }}>
  //             <Form.Label>{nftContractIdFormLabel}</Form.Label>
  //             <Input css={styles.input}>
  //               <Input.Field
  //                 id='nftAssetIdSell'
  //                 onChange={(e) => onChange('nftAssetIdSell', e.target.value)}
  //                 placeholder="0x000...000"
  //                 value={nftAssetIdValue}
  //               />
  //             </Input>
  //           </Form.Control>
  //         </Flex>
  //       ) : (
  //         <Form.Control isRequired css={{ minWidth: "100%" }}>
  //           <Form.Label>{tokenAmountLabel}</Form.Label>
  //           <Input>
  //             <Input.Number
  //               id='assetAmountSell'
  //               allowedDecimalSeparators={[".", ","]}
  //               allowNegative={false}
  //               autoComplete="off"
  //               inputMode="decimal"
  //               decimalScale={DECIMAL_UNITS}
  //               onChange={(e) => onChange('assetAmountSell', e.target.value)}
  //               placeholder="0.0"
  //               thousandSeparator={false}
  //               value={assetAmountValue}
  //             />
  //           </Input>
  //         </Form.Control>
  //       )}
  //     </Flex>
  //     <Flex align="start" css={{ marginTop: "$9" }}>
  //       <AuctionAssetDropdown onChange={handleAssetChange} />
  //     </Flex>
  //   </Flex>
  // );
};

const styles = {
  input: cssObj({
    alignSelf: "stretch",
  }),
};
