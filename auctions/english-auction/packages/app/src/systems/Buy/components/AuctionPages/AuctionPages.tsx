import { Pagination, Heading, Stack } from "@fuel-ui/react";
import { bn } from "fuels";
import { useState } from "react";

import { AuctionPage } from "../AuctionPage/AuctionPage";

import type { AuctionOutput } from "~/types/contracts/AuctionContractAbi";
import type { Option } from "~/types/contracts/common";

interface AuctionPagesProps {
  auctions: Option<AuctionOutput>[];
}

export const AuctionPages = ({ auctions }: AuctionPagesProps) => {
  const [currentPageNumber, setCurrentPageNumber] = useState(1);

  if (auctions.length === 0) {
    return <Heading>No Auctions</Heading>;
  }

  const currentAuction = auctions[currentPageNumber - 1]!;

  return (
    <Stack>
      <AuctionPage
        currentAuction={currentAuction}
        auctionId={bn(currentPageNumber - 1)}
      />
      <Pagination
        pagesCount={auctions.length}
        onPageChange={setCurrentPageNumber}
      >
        <Pagination.Prev />
        <Pagination.Items />
        <Pagination.Next />
      </Pagination>
    </Stack>
  );
};
