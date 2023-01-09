import type { Meta } from "@storybook/react";

import { CreateAuctionPage } from "./CreateAuctionPage";

import { Providers } from "~/systems/Core";

export default {
  component: CreateAuctionPage,
  title: "Auction/pages/CreateAuctionPage",
} as Meta;

export const Usage = () => {
  return (
    <Providers>
      <CreateAuctionPage />
    </Providers>
  );
};
