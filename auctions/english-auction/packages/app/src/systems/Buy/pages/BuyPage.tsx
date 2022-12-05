import { Card, Heading, Stack, Flex } from "@fuel-ui/react";

import { MainLayout } from "~/systems/Core/components/MainLayout";
import { AuctionInfo } from "../components";
import { useAllAuctionInfo } from "../hooks/useAllAuctionInfo";

export function BuyPage() {
  const auctionInfo = useAllAuctionInfo();

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
