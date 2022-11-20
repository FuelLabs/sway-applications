import { Button, Card, Flex, Input, Stack } from "@fuel-ui/react";
import { useState } from "react";

import { useCreateAuction } from "../hooks/useCreateAuction";

import { AuctionAssetInput } from "./AuctionAssetInput";

export const CreateAuction = () => {
  const [auctionValues, setAuctionValues] = useState({
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
    bidAsset: auctionValues.assetIdBid
      ? {
          TokenAsset: {
            amount: auctionValues.assetAmountBid,
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
    initialPrice: auctionValues.initialPrice,
    reservePrice: auctionValues.reservePrice,
    seller: { Address: { value: auctionValues.seller } },
    sellAsset: auctionValues.assetIdSell
      ? {
          TokenAsset: {
            amount: auctionValues.assetAmountSell,
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
    console.log("here: ");
    console.log("field: ", field);
    console.log("value: ", value);
    console.log("vals: ", auctionValues);
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
              onChange={(e) => handleInputChange("duration", e.target.value)}
              placeholder="Duration"
            />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Number
              placeholder="Initial Price"
              onChange={(e) =>
                handleInputChange("initialPrice", e.target.value)
              }
            />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Number
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
