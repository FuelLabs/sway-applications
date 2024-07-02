import { useQuery } from "@tanstack/react-query";
import type { PinataMetadataFilter, PinataPin } from "@pinata/sdk";

import { NFTQueryKeys } from "@/queryKeys";

// We need this custom type bc pinata does not return the same type
// as the type they have defined in ts
export type NFTData = Omit<PinataPin, "metadata"> & {
  metadata: { name?: string; keyvalues: { [key: string]: string | undefined } };
};

export const useGetNFTData = (filter?: PinataMetadataFilter) => {
  const query = useQuery({
    queryKey: [NFTQueryKeys.nftData, filter],
    queryFn: async () => {
      const res = await fetch(
        `/api/files/${filter ? JSON.stringify(filter) : ""}`,
        { method: "GET" }
      );
      if (res.ok) {
        const nftData = await res.json();
        return nftData;
      }
      return [];
    },
  });

  return {
    ...query,
    nftData: (query.data || []) as NFTData[],
  };
};
