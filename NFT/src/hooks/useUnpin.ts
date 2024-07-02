import { useMutation } from "@tanstack/react-query";
import toast from "react-hot-toast";

export const useUnpin = () => {
  const mutation = useMutation({
    mutationFn: async ({
      ipfsHash,
    }: {
      ipfsHash: string;
    }) => {
      await fetch("/api/unpin", {
        method: "POST",
        body: JSON.stringify({ ipfsHash }),
      });
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
