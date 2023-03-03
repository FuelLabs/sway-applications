import { Card, Heading, Stack } from "@fuel-ui/react";

import { AuctionInfo } from "../components";
import { useAllAuctionInfo } from "../hooks/useAllAuctionInfo";

import { MainLayout } from "~/systems/Core/components/MainLayout";
import { AuctionPages } from "../components/AuctionPages";

export function BuyPage() {
  const auctionInfo = useAllAuctionInfo() || [];

  return (
    <MainLayout>
      <Stack align="center">
        <Card align="center" css={{ width: "700px" }}>
          <Heading as="h3">Auctions</Heading>
        </Card>

        <AuctionPages auctions={auctionInfo} />
      </Stack>
    </MainLayout>
  );
}
