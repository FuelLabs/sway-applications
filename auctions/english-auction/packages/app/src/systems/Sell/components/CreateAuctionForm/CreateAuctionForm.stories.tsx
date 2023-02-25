import { Box } from "@fuel-ui/react";

import { MOCK_ACCOUNTS } from "../../__mocks__/accounts";
import { MOCK_ASSETS } from "../../__mocks__/assets";
import { useCreateAuctionForm } from "../../hooks/useCreateAuctionForm";

import { CreateAuctionForm } from "./CreateAuctionForm";

export default {
  component: CreateAuctionForm,
  title: "Auction/components/CreateAuctionForm",
};

export const Usage = () => {
  const form = useCreateAuctionForm();
  return (
    <Box css={{ width: 320 }}>
      <CreateAuctionForm
        form={form}
        walletAddress={MOCK_ACCOUNTS[0].address}
        assets={MOCK_ASSETS}
      />
    </Box>
  );
};
