import {
  Button,
  Card,
  Flex,
  Stack,
  Form,
  Checkbox,
} from "@fuel-ui/react";
import { bn, DECIMAL_UNITS } from "fuels";
import { useEffect, useState } from "react";

import { useCreateAuction } from "../hooks/useCreateAuction";

import { BidAuctionAssetInput } from "./BidAuctionAssetInput";
import { IdentityFormInput } from "./IdentityFormInput";
import { NumericFormInput } from "./NumericFormInput";
import { SellAuctionAssetInput } from "./SellAuctionAssetInput";

import { useAssets } from "~/systems/Core/hooks/useAssets";
import { AddressInput } from "./AddressInput";

export const CreateAuction = () => {
  const [hasReservePrice, setHasReservePrice] = useState(false);

  // Get initial asset ids for the bid and sell assets
  const assets = useAssets();

  const getInitialAssetId = () => {
    if (!!assets && assets.length > 0) {
      return assets[0].assetId;
    }
    return "";
  };

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
    setAuctionValues({
      ...auctionValues,
      assetIdSell: updateAssetId,
      assetIdBid: updateAssetId,
    });
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
    reservePrice: auctionValues.reservePrice.length
      ? bn.parseUnits(auctionValues.reservePrice, DECIMAL_UNITS)
      : undefined,
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
    const isSellAssetFilled =
      !!auctionValues.assetAmountSell.length ||
      (!!auctionValues.assetIdSell.length &&
        !!auctionValues.nftTokenIdSell.length);
    const isReservePriceFilled =
      !hasReservePrice || !!auctionValues.reservePrice.length;
    return (
      !createAuctionMutation.isLoading &&
      !!auctionValues.seller.length &&
      isSellAssetFilled &&
      isReservePriceFilled &&
      !!auctionValues.initialPrice.length &&
      !!auctionValues.duration.length
    );
  };

  return (
    <Flex justify="center">
      <Card>
        <Card.Header>Create Auction</Card.Header>
        <Stack css={{ width: "475px", margin: "10px" }}>
          <AddressInput
            onChange={handleInputChange}
            identityValue={auctionValues.seller}
            objKey="seller"
            label="Seller"
          />

          <SellAuctionAssetInput
            assets={assets!}
            onChange={handleInputChange}
            nftTokenIdValue={auctionValues!.nftTokenIdSell}
            assetAmountValue={auctionValues!.assetAmountSell}
            nftAssetIdValue={auctionValues!.nftAssetIdSell}
          />

          <NumericFormInput
            onChange={handleInputChange}
            formLabel="Initial Price"
            formValue={auctionValues.initialPrice}
            objKey="initialPrice"
            isRequired={true}
            isInvalid={parseFloat(auctionValues.initialPrice) === 0}
            formErrorMessage="Initial price must be greater than 0"
          />

          <Form.Control css={{ flexDirection: "row" }}>
            <Checkbox
              onCheckedChange={() => setHasReservePrice(!hasReservePrice)}
            />
            <Form.Label>Set reserve price</Form.Label>
          </Form.Control>

          {hasReservePrice && (
            <NumericFormInput
              onChange={handleInputChange}
              formLabel="Reserve Price"
              formValue={auctionValues.reservePrice}
              objKey="reservePrice"
              isInvalid={
                parseFloat(auctionValues.reservePrice) <
                parseFloat(auctionValues.initialPrice)
              }
              formErrorMessage="Reserve price cannot be less than the inital price"
            />
          )}

          <BidAuctionAssetInput
            assets={assets!}
            onChange={handleInputChange}
            nftAssetIdValue={auctionValues!.nftAssetIdBid}
          />

          <NumericFormInput
            onChange={handleInputChange}
            formLabel="Duration"
            formValue={auctionValues.duration}
            objKey="duration"
            isRequired={true}
            isInvalid={auctionValues.duration === "0"}
            formErrorMessage="Duration must be greater than 0"
          />

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
