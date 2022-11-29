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
        <Stack css={{ width: "475px", margin: "10px" }}>
          <AuctionAssetInput
            nftContractIdFormLabel="Bid NFT Contract Id"
            tokenAmountLabel="Bid Asset Amount"
            nftIdFormLabel="Bid NFT Id"
            onChange={handleInputChange}
            tokenIdValue={auctionValues!.tokenIdBid}
            assetAmountValue={auctionValues!.assetAmountBid}
            assetIdValue={auctionValues!.assetIdBid}
            id="Bid"
          />
          <Form.Control isRequired>
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
          </Form.Control>

          <Form.Control isRequired>
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
          </Form.Control>

          <Form.Control isRequired>
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
          </Form.Control>

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
