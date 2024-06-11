import { useQuery } from "@tanstack/react-query";
import type { PinataPin } from "@pinata/sdk";

// We need this custom type bc pinata does not return the same type
// as the type they have defined in ts
type NFTData = Omit<PinataPin, "metadata"> & {
  metadata: { name?: string; keyvalues: { [key: string]: string | undefined } };
};

export const useGetNFTData = () => {
  const query = useQuery({
    queryKey: ["getNFTData"],
    queryFn: async () => {
      const res = await fetch("/api/files", { method: "GET" });
      const nftData = await res.json();
      return nftData;
    },
  });

  return {
    ...query,
    nftData: (query.data || []) as NFTData[],
  };
};
