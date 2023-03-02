import { NativeAssetId, bn } from "fuels";

import type { AuctionOutput } from "~/types/contracts/AuctionContractAbi";
import type { Option } from "~/types/contracts/common";

export const MOCK_AUCTIONS: Option<AuctionOutput>[] = [
  {
    bid_asset: {
      TokenAsset: {
        amount: bn(10),
        asset_id: {
          value: NativeAssetId,
        },
      },
    },
    end_block: bn(1000),
    highest_bidder: undefined,
    initial_price: bn(1),
    reserve_price: undefined,
    sell_asset: {
      TokenAsset: {
        amount: bn(10),
        asset_id: {
          value: NativeAssetId,
        },
      },
    },
    seller: { Address: { value: "fuel0x2c8e117bcfba11c76d7db2d43464b1d2093474ef" }},
    state: { Closed: [] },
  },
];
