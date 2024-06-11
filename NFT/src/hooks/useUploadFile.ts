import { useMutation } from "@tanstack/react-query";
import toast from "react-hot-toast";

// TODO: we should save the file locally
// and only pin to ipfs if user deploys nft contract
export const useUploadFile = () => {
  const mutation = useMutation({
    mutationFn: async ({
      fileToUpload,
    }: {
      fileToUpload: File;
    }) => {
      const formData = new FormData();
      formData.append("file", fileToUpload);
      const res = await fetch("/api/files", {
        method: "POST",
        body: formData,
      });
      const ipfsHash = await res.text();
      return ipfsHash;
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
