import { useState } from "react";
import { BoxCentered, Heading, Stack, toast } from "@fuel-ui/react";
import { AssetInput, ArbiterInput, IdentityInput } from "../../contracts/EscrowContractAbi";
import { ArbiterPage, AssetPage, BuyerDeadlinePage } from "./pages";
import { validateAddress, validateContractId } from "../common/utils";
import { useContract } from "../core/hooks";

export function CreateEscrow() {
  const [arbiterAddress, setArbiterAddress] = useState("")
  const [arbiterAsset, setArbiterAsset] = useState("")
  const [arbiterAssetAmount, setArbiterAssetAmount] = useState(0);
  const [deadline, setDeadline] = useState(0);
  const [buyerAddress, setBuyerAddress] = useState("");
  const [recipient, setRecipient] = useState("address");
  const [assets, setAssets] = useState<AssetInput[]>([{ amount: 0, id: { value: "" } }]);
  const [page, setPage] = useState(1)
  const contract = useContract()

  async function createEscrow() {
    // TODO: arbiter is an identity, use one more state variable

    let arbiterIdentity: IdentityInput;
    let buyerIdentity: IdentityInput;

    if (recipient === "address") {
      let { address: user, isError } = validateAddress(arbiterAddress);
      if (isError) return;

      arbiterIdentity = { Address: { value: user } };
    } else {
      let { address: user, isError } = validateContractId(arbiterAddress);
      if (isError) return;

      arbiterIdentity = { ContractId: { value: user } };
    }

    if (recipient === "address") {
      let { address: user, isError } = validateAddress(buyerAddress);
      if (isError) return;

      buyerIdentity = { Address: { value: user } };
    } else {
      let { address: user, isError } = validateContractId(buyerAddress);
      if (isError) return;

      buyerIdentity = { ContractId: { value: user } };
    }

    let arbiter: ArbiterInput = {
      address: arbiterIdentity,
      asset: { value: arbiterAsset },
      fee_amount: arbiterAssetAmount
    }

    await contract!.functions
      .create_escrow(arbiter, assets, buyerIdentity, deadline)
      .call()
      .then(
        (success) => {
          toast.success("Created a new escrow!", { duration: 10000 });
        },
        (error) => {
          if (error.logs === undefined || error.logs.length === 0) {
            toast.error("Unknown error occurred during contract call.", {
              duration: 10000,
            });
          } else {
            toast.error(`Error: ${Object.keys(error.logs[0])[0]}`, {
              duration: 10000,
            });
          }
        }
      );
  }

  return (
    <BoxCentered css={{ color: "$blackA12", fontWeight: "$semibold" }}>
      
      <Stack css={{ width: "100%" }}>
        <Heading css={{ marginLeft: "auto", marginRight: "auto", color: "$pink6", marginBottom: "$14" }}>
          Create Escrow
        </Heading>

        {
          page === 1 && 
          <ArbiterPage 
            setArbiter={setArbiterAddress} 
            setAsset={setArbiterAsset} 
            setAssetAmount={setArbiterAssetAmount} 
            setPage={setPage} 
            currentPage={page}
          />
        }

        {
          page === 2 && 
          <BuyerDeadlinePage 
            setBuyer={setBuyerAddress} 
            setDeadline={setDeadline} 
            setRecipient={setRecipient} 
            setPage={setPage} 
            currentPage={page}
          />
        }

        {
          page == 3 &&
          <AssetPage 
            setAssets={setAssets} 
            setPage={setPage}
            currentPage={page}
            assets={assets}
            createEscrow={createEscrow}
          />
        }
      </Stack>

    </BoxCentered>
  );
}
