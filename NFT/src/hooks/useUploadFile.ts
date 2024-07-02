import { useMutation } from "@tanstack/react-query";
import toast from "react-hot-toast";

import { useCreateNFT } from "./useCreateNFT";

type UploadFileParams = {
  fileToUpload: File;
  name: string;
  description: string;
  symbol: string;
};

export const useUploadFile = () => {
  const createNFT = useCreateNFT();

  const mutation = useMutation({
    mutationFn: async ({ fileToUpload }: UploadFileParams) => {
      const formData = new FormData();
      formData.append("file", fileToUpload);
      const res = await fetch("/api/files", {
        method: "POST",
        body: formData,
      });
      const ipfsHash = await res.text();
      return ipfsHash;
    },
    onSuccess: (data, { name, description, symbol }) => {
      const newCid = data;
      createNFT.mutateAsync({
        cid: newCid,
        name,
        description,
        symbol,
      });
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
