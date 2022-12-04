import { Card, Heading, Stack, Flex, Button } from "@fuel-ui/react";
import { MainLayout } from "~/systems/Core/components/MainLayout";
import { EndBlock } from "../components/EndBlock";
import { useAllAuctionInfo } from "../hooks/useAllAuctionInfo";

export function BuyPage() {
  const auctionInfo = useAllAuctionInfo();

  const auctions = auctionInfo?.map((auction) => {
    return (
      <Flex>
        <div>
          <Heading as="h5">Seller</Heading>
          <div>{auction?.seller.Address?.value}</div>

          <Heading as="h5">Sell Asset</Heading>
          <div>{auction?.sell_asset.TokenAsset?.amount.toString()}</div>
          <div>{auction?.sell_asset.TokenAsset?.asset_id.value}</div>

          <Heading as="h5">Initial Price</Heading>
          <div>{auction?.initial_price.toString()}</div>

          <Heading as="h5">Bid Asset</Heading>
          <div>{auction?.bid_asset.TokenAsset?.asset_id.value}</div>
          <div>{auction?.bid_asset.TokenAsset?.amount.toString()}</div>

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
        <Card css={{ width: "950px" }}>
          <Card.Header>
            <Heading as="h3">
              Auctions
            </Heading>
          </Card.Header>
          <Card.Body>
            <Stack>
              {auctions}
              <Button>Bid on Auction</Button>
              <Button>Cancel Auction</Button>
            </Stack>
          </Card.Body>
        </Card>
      </Flex>
    </MainLayout>
  );
}
