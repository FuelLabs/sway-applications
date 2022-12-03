import { Button, Card, Flex, Input, Stack, Form, Checkbox } from "@fuel-ui/react";
import { bn, DECIMAL_UNITS } from "fuels";
import { useEffect, useState } from "react";

import { useCreateAuction } from "../hooks/useCreateAuction";

import { SellAuctionAssetInput } from "./SellAuctionAssetInput";
import { BidAuctionAssetInput } from "./BidAuctionAssetInput";
import { useAssets } from "~/systems/Core/hooks/useAssets";
import { AuctionAssetDropdown } from "./AuctionAssetDropdown";

export const CreateAuction = () => {
  const [hasReservePrice, setHasReservePrice] = useState(false);

  // Get initial asset ids for the bid and sell assets
  const assets = useAssets();

  const getInitialAssetId = () => {
    if (!!assets && assets.length > 0) {
      return assets[0].assetId;
    }
    return "";
  }



  const initialAssetId = getInitialAssetId();

  const [auctionValues, setAuctionValues] = useState<{
    assetIdBid: string;
    assetAmountBid: string;
    nftTokenIdBid: string;
    nftAssetIdBid: string;
    duration: string;
    initialPrice: string;
    reservePrice: string;
    seller: string;
    assetIdSell: string;
    assetAmountSell: string;
    nftTokenIdSell: string;
    nftAssetIdSell: string;
  }>({
    assetIdBid: initialAssetId,
    assetAmountBid: "",
    nftTokenIdBid: "",
    nftAssetIdBid: "",
    duration: "",
    initialPrice: "",
    reservePrice: "",
    seller: "",
    assetIdSell: initialAssetId,
    assetAmountSell: "",
    nftTokenIdSell: "",
    nftAssetIdSell: "",
  });

  useEffect(() => {
    const updateAssetId = getInitialAssetId();
    setAuctionValues({ ...auctionValues, "assetIdSell": updateAssetId, "assetIdBid": updateAssetId });
  }, [assets]);

  // TODO refactor: figure out how to make this look nicer
  const createAuctionMutation = useCreateAuction({
    bidAsset: !auctionValues.nftAssetIdBid
      ? {
        TokenAsset: {
          amount: bn(0),
          asset_id: { value: auctionValues.assetIdBid },
        },
      }
      : {
        NFTAsset: {
          token_id: bn(0),
          asset_id: { value: auctionValues.nftAssetIdBid },
        },
      },
    duration: auctionValues.duration,
    initialPrice: bn.parseUnits(auctionValues.initialPrice, DECIMAL_UNITS),
    reservePrice: !!auctionValues.reservePrice.length ? bn.parseUnits(auctionValues.reservePrice, DECIMAL_UNITS) : undefined,
    sellerAddress: auctionValues.seller,
    sellAsset: !auctionValues.nftAssetIdSell
      ? {
        TokenAsset: {
          amount: bn.parseUnits(auctionValues.assetAmountSell, DECIMAL_UNITS),
          asset_id: { value: auctionValues.assetIdSell },
        },
      }
      : {
        NFTAsset: {
          token_id: auctionValues.nftTokenIdSell,
          asset_id: { value: auctionValues.nftAssetIdSell },
        },
      },
  });

  const handleInputChange = (field: string, value: string) => {
    setAuctionValues({ ...auctionValues!, [field]: value });
  };

  // TODO fix: doesn't account for invalid inputs
  // TODO fix: doesn't account for bid asset input
  const canCreateAuction = () => {
    const isSellAssetFilled = !!auctionValues.assetAmountSell.length || (!!auctionValues.assetIdSell.length && !!auctionValues.nftTokenIdSell.length);
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

          <SellAuctionAssetInput
            assets={assets!}
            nftContractIdFormLabel="Sell NFT Contract Id"
            nftIdFormLabel="Sell NFT Id"
            onChange={handleInputChange}
            nftTokenIdValue={auctionValues!.nftTokenIdSell}
            assetAmountValue={auctionValues!.assetAmountSell}
            nftAssetIdValue={auctionValues!.nftAssetIdSell}
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

          <BidAuctionAssetInput assets={assets!} onChange={handleInputChange} nftAssetIdValue={auctionValues!.nftAssetIdBid} />

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
