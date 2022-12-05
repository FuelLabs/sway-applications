import { Card, Heading, Stack, Flex, Button, Input } from "@fuel-ui/react";
import { AssetAmountInput } from "~/systems/Core/components/AssetAmountInput";
import { AssetIdOutput } from "~/systems/Core/components/AssetIdOutput";
import { AssetOutput } from "~/systems/Core/components/AssetOutput";

import { MainLayout } from "~/systems/Core/components/MainLayout";
import { getSlicedAddress } from "~/systems/Core/utils";
import { AuctionInfo } from "../components";

import { EndBlock } from "../components/EndBlock";
import { PlaceBid } from "../components/PlaceBid";
import { useAllAuctionInfo } from "../hooks/useAllAuctionInfo";

export function BuyPage() {
  const auctionInfo = useAllAuctionInfo();

  const auctions = auctionInfo?.map((auction) => {
    return (
      <Stack>
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

          <Flex>
            <AssetIdOutput
              assetId={getSlicedAddress(auction?.seller.Address?.value!)}
              heading="Seller"
            />

            <AssetIdOutput
              assetId={(auction?.highest_bidder?.Address?.value && getSlicedAddress(auction?.highest_bidder?.Address?.value)) || "None"}
              heading="Highest Bidder"
            />
          </Flex>

          <EndBlock endBlock={auction!.end_block} />
          <Button css={{ minWidth: "100%"}}>Cancel Auction</Button>
      </Stack>
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
              <AuctionInfo auctions={auctionInfo!} />
            </Stack>
          </Card.Body>
        </Card>
      </Flex>
    </MainLayout>
  );
}
