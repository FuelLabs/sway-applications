import { Card, Heading, Stack, Flex, Button, Input } from "@fuel-ui/react";
import { AssetAmountInput } from "~/systems/Core/components/AssetAmountInput";
import { AssetOutput } from "~/systems/Core/components/AssetOutput";

import { MainLayout } from "~/systems/Core/components/MainLayout";
import { getAssetText } from "~/systems/Core/utils";

import { EndBlock } from "../components/EndBlock";
import { PlaceBid } from "../components/PlaceBid";
import { useAllAuctionInfo } from "../hooks/useAllAuctionInfo";

export function BuyPage() {
  const auctionInfo = useAllAuctionInfo();

  const auctions = auctionInfo?.map((auction) => {
    return (
      <Flex>
        <div>
          <Flex>
            <AssetOutput
              assetId={auction?.sell_asset.TokenAsset?.asset_id.value!}
              assetAmount={auction?.sell_asset.TokenAsset?.amount.format()!}
              heading="Selling"
            />

            <AssetOutput
              assetId={auction?.bid_asset.TokenAsset?.asset_id.value!}
              assetAmount={auction?.bid_asset.TokenAsset?.amount.format()!}
              heading="Highest Bid"
            />

            <AssetOutput
              assetId={auction?.bid_asset.TokenAsset?.asset_id.value!}
              assetAmount={auction?.initial_price.format()!}
              heading="Initial Price"
            />
          </Flex>

          <PlaceBid />

          <Heading as="h5">Seller</Heading>
          <div>{auction?.seller.Address?.value}</div>

          <Heading as="h5">Highest Bidder</Heading>
          <div>{auction?.highest_bidder?.Address?.value || "None"}</div>

          <EndBlock endBlock={auction!.end_block} />

          <div>{auction?.highest_bidder?.Address?.value}</div>
          <div>{auction?.highest_bidder?.ContractId?.value}</div>
          <div>{auction?.reserve_price?.toString()}</div>

          <div>{auction?.state.Closed}</div>
          <div>{auction?.state.Open}</div>
        </div>
      </Flex>
    );
  });

  return (
    <MainLayout>
      <Flex justify="center">
        <Card css={{ width: "600px" }}>
          <Card.Header>
            <Heading as="h3">
              Auctions
            </Heading>
          </Card.Header>
          <Card.Body>
            <Stack>
              {auctions}
              <Button>Cancel Auction</Button>
            </Stack>
          </Card.Body>
        </Card>
      </Flex>
    </MainLayout>
  );
}
