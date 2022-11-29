import { Button, Card, Flex, Input, Stack, Form } from "@fuel-ui/react";
import { bn, DECIMAL_UNITS } from "fuels";
import { useState } from "react";

import { useCreateAuction } from "../hooks/useCreateAuction";

import { AuctionAssetInput } from "./AuctionAssetInput";

export const CreateAuction = () => {
  const [auctionValues, setAuctionValues] = useState<{
    assetIdBid: string;
    assetAmountBid: string;
    tokenIdBid: string;
    tokenTypeBid: string;
    duration: string;
    initialPrice: string;
    reservePrice: string;
    seller: string;
    assetIdSell: string;
    assetAmountSell: string;
    tokenIdSell: string;
    tokenTypeSell: string;
  }>({
    assetIdBid: "",
    assetAmountBid: "",
    tokenIdBid: "",
    tokenTypeBid: "",
    duration: "",
    initialPrice: "",
    reservePrice: "",
    seller: "",
    assetIdSell: "",
    assetAmountSell: "",
    tokenIdSell: "",
    tokenTypeSell: "",
  });

  const createAuctionMutation = useCreateAuction({
    bidAsset: !auctionValues.assetIdBid
      ? {
        TokenAsset: {
          amount: bn.parseUnits(auctionValues.assetAmountBid, DECIMAL_UNITS),
          asset_id: { value: auctionValues.tokenTypeBid },
        },
      }
      : {
        NFTAsset: {
          token_id: auctionValues.tokenIdBid,
          asset_id: { value: auctionValues.assetIdBid },
        },
      },
    duration: auctionValues.duration,
    initialPrice: bn.parseUnits(auctionValues.initialPrice, DECIMAL_UNITS),
    reservePrice: bn.parseUnits(auctionValues.reservePrice, DECIMAL_UNITS),
    sellerAddress: auctionValues.seller,
    sellAsset: !auctionValues.assetIdSell
      ? {
        TokenAsset: {
          amount: bn.parseUnits(auctionValues.assetAmountSell, DECIMAL_UNITS),
          asset_id: { value: auctionValues.tokenTypeSell },
        },
      }
      : {
        NFTAsset: {
          token_id: auctionValues.tokenIdSell,
          asset_id: { value: auctionValues.assetIdSell },
        },
      },
  });

  const handleInputChange = (field: string, value: string) => {
    setAuctionValues({ ...auctionValues!, [field]: value });
  };

  return (
    <Flex justify="center">
      <Card>
        <Card.Header>Create Auction</Card.Header>
        <Stack css={{ width: "475px", margin: "10px", alignItems: "center" }}>
            <AuctionAssetInput
              placeholderAssetId="Bid Asset Id"
              placeholderTokenAmount="Bid Asset Amount"
              placeholderTokenId="Bid Token Id"
              onChange={handleInputChange}
              tokenIdValue={auctionValues!.tokenIdBid}
              assetAmountValue={auctionValues!.assetAmountBid}
              assetIdValue={auctionValues!.assetIdBid}
              id="Bid"
            />
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Number
              inputMode="numeric"
              allowNegative={false}
              onChange={(e) => handleInputChange("duration", e.target.value)}
              placeholder="Duration"
            />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Number
              allowNegative={false}
              placeholder="Initial Price"
              onChange={(e) =>
                handleInputChange("initialPrice", e.target.value)
              }
            />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Number
              allowNegative={false}
              placeholder="Reserve Price"
              onChange={(e) =>
                handleInputChange("reservePrice", e.target.value)
              }
            />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Field
              placeholder="Seller"
              onChange={(e) => handleInputChange("seller", e.target.value)}
            />
          </Input>
          <AuctionAssetInput
            placeholderAssetId="Sell Asset Id"
            placeholderTokenAmount="Sell Asset Amount"
            placeholderTokenId="Sell Token Id"
            onChange={handleInputChange}
            tokenIdValue={auctionValues!.tokenIdSell}
            assetAmountValue={auctionValues!.assetAmountSell}
            assetIdValue={auctionValues!.assetIdSell}
            id="Sell"
          />
          <Button
            leftIcon="Plus"
            onPress={() => createAuctionMutation.mutate()}
          >
            Create Auction
          </Button>
        </Stack>
      </Card>
    </Flex>
  );
};
