import { Button } from "@/components/Button";
import { Input } from "@/components/Input";
import { UploadButton } from "@/components/UploadButton";
import { useCreateNFT } from "@/hooks/useCreateNFT";
import { Box, IconButton, Stack, Typography } from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";
import { useState } from "react";

import { GATEWAY_URL } from "@/lib";

export default function Create() {
  const [cid, setCid] = useState("");
  const [file, setFile] = useState<File>();
  const [name, setName] = useState("");
  const [symbol, setSymbol] = useState("");
  const [numberOfCopies, setNumberOfCopies] = useState<number>();

  console.log(`file`, file);

  const createNFT = useCreateNFT();

  // TODO: add way to change file
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
            <IconButton sx={{ color: "white", alignSelf: "end", padding: "0px", marginRight: "-30px", marginTop: "-10px" }}>
              <CloseIcon />
            </IconButton>
            <img src={URL.createObjectURL(file)} />
          </>
        ) : (
          <>
            <Typography>
              TODO: add info about supported files types and size
            </Typography>
            <UploadButton setCid={setCid} setFile={setFile}/>
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
      <Typography>Number of copies</Typography>
      <Input
        value={numberOfCopies?.toString()}
        onChange={(event) => setNumberOfCopies(Number(event.target.value))}
        type="number"
        placeholder="10"
      />
      <Button
        onClick={() => {
          createNFT.mutate({
            cid,
            name,
            symbol,
            numberOfCopies: numberOfCopies || 0,
          });
        }}
      >
        Create NFT
      </Button>
    </Stack>
  );
}
