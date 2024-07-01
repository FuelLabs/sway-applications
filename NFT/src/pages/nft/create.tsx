import { UploadButton } from "@/components/UploadButton";
import { useUploadFile } from "@/hooks/useUploadFile";
import { IconButton, Stack, TextField } from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";
import { useState } from "react";
import clsx from "clsx";
import { useIsMutating } from "@tanstack/react-query";

import { Button } from "@/components/Button";
import { Input, inputStyle } from "@/components/Input";
import { useActiveWallet } from "@/hooks/useActiveWallet";
import { NFTImage } from "@/components/NFTImage";
import { Text } from "@/components/Text";
import { NFTQueryKeys } from "@/queryKeys";

export default function Create() {
  const [file, setFile] = useState<File>();
  const [name, setName] = useState("");
  const [symbol, setSymbol] = useState("");
  const [description, setDescription] = useState("");
  const { isConnected, isPending } = useActiveWallet();

  const isCreatingNFT = Boolean(useIsMutating({
    mutationKey: [NFTQueryKeys.createNFT],
  }));
  const uploadFile = useUploadFile();

  const isCreateButtonDisabled =
    !name || !symbol || uploadFile.isPending || isCreatingNFT;

  return (
    <>
      {isPending ? (
        <Text>Loading...</Text>
      ) : isConnected ? (
        <div className="gradient-border rounded-2xl">
          <div className="grain rounded-2xl p-1.5 drop-shadow-xl">
            <Stack
              spacing={2}
              className={clsx(
                "gradient-border",
                "h-full",
                "rounded-xl",
                "bg-gradient-to-b",
                "from-zinc-900",
                "to-zinc-950/80",
                "px-4",
                "py-8"
              )}
            >
              <Text variant="h4" sx={{ paddingBottom: "28px" }}>
                Create New NFT
              </Text>
              <Text>Upload File</Text>
              <Stack
                alignItems="center"
                justifyContent="space-around"
                sx={{
                  border: "1px dashed",
                  borderColor: "#434343",
                  borderRadius: "15px",
                }}
                className="px-8 pb-8 pt-6"
              >
                {file ? (
                  <>
                    <IconButton
                      onClick={() => setFile(undefined)}
                      sx={{
                        color: "white",
                        alignSelf: "end",
                        padding: "0px",
                        marginRight: "-30px",
                        marginTop: "-10px",
                      }}
                    >
                      <CloseIcon />
                    </IconButton>
                    <NFTImage src={URL.createObjectURL(file)} className="w-64 h-64 md:w-80 md:h-80 lg:w-96 lg:h-96" />
                  </>
                ) : (
                  <Stack spacing={2}>
                    <Text>
                      Recommended size: 350 x 350. File types supported: JPG,
                      PNG, or GIF.
                    </Text>
                    <UploadButton setFile={setFile} />
                  </Stack>
                )}
              </Stack>
              <Text>Name</Text>
              <Input
                value={name}
                onChange={(event) => setName(event.target.value)}
                placeholder="Buff Dragons"
              />
              <Text>Symbol</Text>
              <Input
                value={symbol}
                onChange={(event) => setSymbol(event.target.value)}
                placeholder="BD"
              />
              <Text>Description (Optional)</Text>
              <TextField
                value={description}
                onChange={(event) => setDescription(event.target.value)}
                placeholder="Cool dragons that like to lift weights"
                multiline
                rows={4}
                className={clsx([...inputStyle])}
                inputProps={{ className: "placeholder:text-zinc-400 text-zinc-50" }}
              />
              <Button
                disabled={isCreateButtonDisabled}
                onClick={() => {
                  if (file) {
                    uploadFile.mutateAsync({
                      fileToUpload: file,
                      name,
                      description,
                      symbol,
                    });
                  }
                }}
              >
                {uploadFile.isPending
                  ? "Uploading to IPFS..."
                  : isCreatingNFT
                    ? "Creating NFT..."
                    : "Create NFT"}
              </Button>
            </Stack>
          </div>
        </div>
      ) : (
        <Text>Please connect your wallet to create an NFT.</Text>
      )}
    </>
  );
}
