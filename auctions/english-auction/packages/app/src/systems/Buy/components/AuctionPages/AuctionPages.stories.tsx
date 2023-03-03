import { Box } from "@fuel-ui/react";

import { MOCK_AUCTIONS } from "../../__mocks__/auctions";

import { AuctionPages } from "./AuctionPages";

import { Providers } from "~/systems/Core";

export default {
  component: AuctionPages,
  title: "Buy/components/AuctionPages",
};

export const ClosedAuction = () => {
  return (
    <Box css={{ width: 320 }}>
      <Providers>
        <AuctionPages auctions={MOCK_AUCTIONS} />
      </Providers>
    </Box>
  );
};

export const WithNoAuctions = () => {
  return (
    <Box css={{ widht: 320 }}>
      <Providers>
        <AuctionPages auctions={[]} />Ï
      </Providers>
    </Box>
  );
};
