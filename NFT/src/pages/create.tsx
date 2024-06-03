import { Button } from "@/components/Button";
import { Input } from "@/components/Input";
import { UploadButton } from "@/components/UploadButton";
import { useCreateNFT } from "@/hooks/useCreateNFT";
import { Box, Stack, Typography } from "@mui/material";
import { useState } from "react";

export default function Create() {
  const [cid, setCid] = useState("");
  const [name, setName] = useState("");
  const [symbol, setSymbol] = useState("");
  const [numberOfCopies, setNumberOfCopies] = useState<number>();

  const createNFT = useCreateNFT();

  return (
    <Stack spacing={2}>
      <Typography variant="h2" sx={{ paddingBottom: "48px" }}>
        Create New NFT
      </Typography>
      <Typography>Upload File</Typography>
      <Stack
        height={150}
        alignItems="center"
        justifyContent="space-around"
        sx={{ border: "1px dashed grey", borderRadius: "15px" }}
      >
        <Typography>
          TODO: add info about supported files types and size
        </Typography>
        <UploadButton setCid={setCid} />
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
