import { Button } from "@/components/Button";
import { Input } from "@/components/Input";
import { UploadButton } from "@/components/UploadButton";
import { useCreateNFT } from "@/hooks/useCreateNFT";
import { useUploadFile } from "@/hooks/useUploadFile";
import { IconButton, Stack, Typography } from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";
import { useState, useEffect } from "react";
import { useUpdateMetadata } from "@/hooks/useUpdateMetadata";

export default function Create() {
  const [cid, setCid] = useState("");
  const [file, setFile] = useState<File>();
  const [name, setName] = useState("");
  const [symbol, setSymbol] = useState("");
  const [description, setDescription] = useState("");
  const [numberOfCopies, setNumberOfCopies] = useState<number>();

  const createNFT = useCreateNFT();
  const uploadFile = useUploadFile();
  const updateMetadata = useUpdateMetadata();

  useEffect(() => {
    if (uploadFile.data) {
      const newCid = uploadFile.data;
      setCid(newCid);
      createNFT.mutate({
        cid: newCid,
        name,
        symbol,
        numberOfCopies: numberOfCopies || 0,
      });
    }
  }, [uploadFile.data]);

  useEffect(() => {
    if (createNFT.data) {
      const nftContractId = createNFT.data;
      updateMetadata.mutate({
        ipfsHash: cid,
        metadata: { keyvalues: { nftContractId: nftContractId.toB256() } },
      });
    }
  }, [createNFT.data]);

  // TODO: unpin file if user does not approve txs
  return (
    <Stack spacing={2}>
      <Typography variant="h2" sx={{ paddingBottom: "48px" }}>
        Create New NFT
      </Typography>
      <Typography>Upload File</Typography>
      <Stack
        alignItems="center"
        justifyContent="space-around"
        padding={5}
        sx={{ border: "1px dashed grey", borderRadius: "15px" }}
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
          <>
            <Typography>
              TODO: add info about supported files types and size
            </Typography>
            <UploadButton setFile={setFile} />
          </>
        )}
      </Stack>
      <Typography>Name</Typography>
      <Input
        value={name}
        onChange={(event) => setName(event.target.value)}
        placeholder="Buff Dragons"
      />
      <Typography>Symbol</Typography>
      <Input
        value={symbol}
        onChange={(event) => setSymbol(event.target.value)}
        placeholder="BD"
      />
      <Typography>Description (Optional)</Typography>
      <Input
        value={description}
        onChange={(event) => setDescription(event.target.value)}
        placeholder="Cool dragons that like to lift weights"
      />
      <Typography>Number of copies</Typography>
      <Input
        value={numberOfCopies?.toString()}
        onChange={(event) => setNumberOfCopies(Number(event.target.value))}
        type="number"
        placeholder="10"
      />
      <Button
        onClick={() => {
          if (file) {
            uploadFile.mutate({
              fileToUpload: file,
              nftName: name,
              nftDescription: description,
            });
          }
        }}
      >
        Create NFT
      </Button>
    </Stack>
  );
}
