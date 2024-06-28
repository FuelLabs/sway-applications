import { useMutation, useQueryClient } from "@tanstack/react-query";
import toast from "react-hot-toast";
import { NFTQueryKeys } from "@/queryKeys";

// WARNING: although this can take arbitrary metadata
// it overwrites the past metadata
export const useUpdateMetadata = () => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: async ({
      metadata,
      ipfsHash,
    }: {
      metadata: { name?: string; keyvalues?: { [key: string]: string } };
      ipfsHash: string;
    }) => {
      await fetch("/api/metadata", {
        method: "PUT",
        body: JSON.stringify({ metadata, ipfsHash }),
      });
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: [NFTQueryKeys.nftData] });
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
