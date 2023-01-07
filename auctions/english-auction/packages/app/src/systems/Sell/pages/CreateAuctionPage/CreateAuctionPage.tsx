import { CreateAuctionForm } from "../../components/CreateAuctionForm";
import { useCreateAuctionForm } from "../../hooks/useCreateAuctionForm";

import { MainLayout, Providers, useAssets, useWallet } from "~/systems/Core";

export const CreateAuctionPage = () => {
  const form = useCreateAuctionForm();
  const wallet = useWallet();
  if (!wallet) throw new Error("Error: no wallet connected");
  const assets = useAssets() || [];

  return (
    <Providers>
      <MainLayout>
        <CreateAuctionForm
          form={form}
          walletAddress={wallet?.address.toHexString()}
          assets={assets}
        />
      </MainLayout>
    </Providers>
  );
};
