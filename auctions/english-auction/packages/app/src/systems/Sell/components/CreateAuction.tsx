import { Button, Card, Flex, Input, Stack, Form, Checkbox } from "@fuel-ui/react";
import { bn, DECIMAL_UNITS } from "fuels";
import { useState } from "react";

import { useCreateAuction } from "../hooks/useCreateAuction";
import { AuctionAssetDropdown } from "./AuctionAssetDropdown";

import { AuctionAssetInput } from "./AuctionAssetInput";
import { BidAuctionAssetInput } from "./BidAuctionAssetInput";

export const CreateAuction = () => {
  const [hasReservePrice, setHasReservePrice] = useState(false);
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
          amount: bn(0),
          asset_id: { value: auctionValues.tokenTypeBid },
        },
      }
      : {
        NFTAsset: {
          token_id: bn(0),
          asset_id: { value: auctionValues.assetIdBid },
        },
      },
    duration: auctionValues.duration,
    initialPrice: bn.parseUnits(auctionValues.initialPrice, DECIMAL_UNITS),
    reservePrice: bn.parseUnits(auctionValues.reservePrice.length ? auctionValues.reservePrice : "0", DECIMAL_UNITS),
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

  // TODO fix: doesn't account for invalid inputs
  // TODO fix: doesn't account for bid asset input
  const canCreateAuction = () => {
    const isSellAssetFilled = !!auctionValues.assetAmountSell.length || (!!auctionValues.assetIdSell.length && !!auctionValues.tokenIdSell.length);
    const isReservePriceFilled = !hasReservePrice || !!auctionValues.reservePrice.length;
    return (
      !createAuctionMutation.isLoading &&
      !!auctionValues.seller.length &&
      isSellAssetFilled &&
      isReservePriceFilled &&
      !!auctionValues.initialPrice.length &&
      !!auctionValues.duration.length
    );
  }

  return (
    <Flex justify="center">
      <Card>
        <Card.Header>Create Auction</Card.Header>
        <Stack css={{ width: "475px", margin: "10px" }}>
          <Form.Control isRequired>
            <Form.Label>
              Seller
            </Form.Label>
            <Input css={{ alignSelf: "stretch" }}>
              <Input.Field
                placeholder="0x000...000"
                onChange={(e) => handleInputChange("seller", e.target.value)}
              />
            </Input>
          </Form.Control>

          <AuctionAssetInput
            nftContractIdFormLabel="Sell NFT Contract Id"
            tokenAmountLabel="Sell Asset Amount"
            nftIdFormLabel="Sell NFT Id"
            onChange={handleInputChange}
            tokenIdValue={auctionValues!.tokenIdSell}
            assetAmountValue={auctionValues!.assetAmountSell}
            assetIdValue={auctionValues!.assetIdSell}
            id="Sell"
          />

          <Form.Control isRequired isInvalid={parseFloat(auctionValues["initialPrice"]) === 0}>
            <Form.Label>
              Initial Price
            </Form.Label>
            <Input css={{ alignSelf: "stretch" }}>
              <Input.Number
                allowNegative={false}
                placeholder="0.0"
                onChange={(e) =>
                  handleInputChange("initialPrice", e.target.value)
                }
              />
            </Input>
            <Form.ErrorMessage>
              Initial price must be greater than 0
            </Form.ErrorMessage>
          </Form.Control>

          <Form.Control css={{ flexDirection: "row" }}>
            <Checkbox onCheckedChange={() => setHasReservePrice(!hasReservePrice)} />
            <Form.Label>
              Set reserve price
            </Form.Label>
          </Form.Control>

          {hasReservePrice && <Form.Control isRequired isInvalid={parseFloat(auctionValues["reservePrice"]) < parseFloat(auctionValues["initialPrice"])}>
            <Form.Label>Reserve Price</Form.Label>
            <Input css={{ alignSelf: "stretch" }}>
              <Input.Number
                allowNegative={false}
                placeholder="0.0"
                onChange={(e) =>
                  handleInputChange("reservePrice", e.target.value)
                }
              />
            </Input>
            <Form.ErrorMessage>
              Reserve price cannot be less than the initial price
            </Form.ErrorMessage>
          </Form.Control>
          }

          <BidAuctionAssetInput onChange={handleInputChange} id='Bid' assetIdValue={auctionValues!.assetIdBid} />

          <Form.Control isRequired isInvalid={auctionValues["duration"] === "0"}>
            <Form.Label>
              Duration
            </Form.Label>
            <Input css={{ alignSelf: "stretch" }}>
              <Input.Number
                inputMode="numeric"
                allowNegative={false}
                onChange={(e) => handleInputChange("duration", e.target.value)}
                placeholder="0"
              />
            </Input>
            <Form.ErrorMessage>
              Duration must be greater than 0
            </Form.ErrorMessage>
          </Form.Control>

          <Button
            isDisabled={!canCreateAuction()}
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
