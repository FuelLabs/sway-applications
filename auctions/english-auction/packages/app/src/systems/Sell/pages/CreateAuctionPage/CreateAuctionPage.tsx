import { CreateAuctionForm } from "../../components/CreateAuctionForm";
import { useCreateAuctionForm } from "../../hooks/useCreateAuctionForm";

import { MainLayout, useAssets, useWallet } from "~/systems/Core";

export const CreateAuctionPage = () => {
  const form = useCreateAuctionForm();
  const { wallet, isLoading, isError } = useWallet();
  if (isError) throw new Error("Error: no wallet connected");
  const assets = useAssets() || [];

  return (
    <MainLayout>
      {isLoading ? (
        <div>Loading...</div>
      ) : (
        <CreateAuctionForm
          form={form}
          walletAddress={wallet!.address.toHexString()!}
          assets={assets}
        />
      )}
    </MainLayout>
  );
};
