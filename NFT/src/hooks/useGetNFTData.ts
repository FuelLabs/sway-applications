import { useQuery } from "@tanstack/react-query";
import type { PinataPin } from "@pinata/sdk";

export const useGetNFTData = () => {

    const query = useQuery({
        queryKey: ["getNFTData"],
        queryFn: async () => {
            const res = await fetch("/api/files", { method: "GET" });
            const nftData = await res.json();
            return nftData as PinataPin[];
        }
    });

    return { ...query, nftData: query.data || []};
};
