import { Stack } from "@fuel-ui/react";

import { AuctionAssetInfo } from "./AuctionAssetInfo";

import type {
  NFTAssetOutput,
  TokenAssetOutput,
} from "~/types/contracts/AuctionContractAbi";

interface AuctionPageProps {
  sellAsset: NFTAssetOutput | TokenAssetOutput;
  sellAssetAmount: string;
  isSellAssetNFT: boolean;
  bidAsset: NFTAssetOutput | TokenAssetOutput;
  bidAssetAmount: string;
  isBidAssetNFT: boolean;
  initialPrice: string;
}

export const AuctionPage = ({
  sellAsset,
  sellAssetAmount,
  isSellAssetNFT,
  bidAsset,
  bidAssetAmount,
  isBidAssetNFT,
  initialPrice
}: AuctionPageProps) => {
  return (
    <Stack>
     <AuctionAssetInfo
        sellAsset={sellAsset}
        sellAssetAmount={sellAssetAmount}
        isSellAssetNFT={isSellAssetNFT}
        bidAsset={bidAsset}
        bidAssetAmount={bidAssetAmount}
        isBidAssetNFT={isBidAssetNFT}
        initialPrice={initialPrice}
     />
    </Stack>
  );
};
