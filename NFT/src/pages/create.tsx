//import { Input } from "@/components/Input";
import { UploadButton } from "@/components/UploadButton";
import { useCreateNFT } from "@/hooks/useCreateNFT";
import { useUploadFile } from "@/hooks/useUploadFile";
import { IconButton, Stack, Typography } from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";
import { useState, useEffect } from "react";
import clsx from "clsx";
import { Button } from "@/components/Button";
import { Input } from "@/components/Input";

export default function Create() {
  const [cid, setCid] = useState("");
  const [file, setFile] = useState<File>();
  const [name, setName] = useState("");
  const [symbol, setSymbol] = useState("");
  const [description, setDescription] = useState("");

  const createNFT = useCreateNFT();
  const uploadFile = useUploadFile();

  useEffect(() => {
    if (uploadFile.data) {
      const newCid = uploadFile.data;
      setCid(newCid);
      createNFT.mutate({
        cid: newCid,
        name,
        description,
        symbol,
      });
    }
  }, [uploadFile.data]);

  const isCreateButtonDisabled = !name || !symbol;

  // TODO: unpin file if user does not approve txs
  return (
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
          <Typography
            variant="h4"
            className={clsx("text-white")}
            sx={{ paddingBottom: "28px" }}
          >
            Create New NFT
          </Typography>
          <Typography className="text-white">Upload File</Typography>
          <Stack
            alignItems="center"
            justifyContent="space-around"
            padding={5}
            sx={{
              border: "1px dashed",
              borderColor: "#434343",
              borderRadius: "15px",
            }}
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
                <img src={URL.createObjectURL(file)} />
              </>
            ) : (
              <Stack spacing={2}>
                <Typography className="text-white">
                  Recommended size: 350 x 350. File types supported: JPG, PNG,
                  or GIF.
                </Typography>
                <UploadButton setFile={setFile} />
              </Stack>
            )}
          </Stack>
          <Typography className="text-white">Name</Typography>
          <Input
            value={name}
            onChange={(event) => setName(event.target.value)}
            placeholder="Buff Dragons"
          />
          <Typography className="text-white">Symbol</Typography>
          <Input
            value={symbol}
            onChange={(event) => setSymbol(event.target.value)}
            placeholder="BD"
          />
          <Typography className="text-white">Description (Optional)</Typography>
          <Input
            value={description}
            onChange={(event) => setDescription(event.target.value)}
            placeholder="Cool dragons that like to lift weights"
          />
          <Button
            disabled={isCreateButtonDisabled}
            onClick={() => {
              if (file) {
                uploadFile.mutate({
                  fileToUpload: file,
                });
              }
            }}
          >
            Create NFT
          </Button>
        </Stack>
      </div>
    </div>
  );
}
