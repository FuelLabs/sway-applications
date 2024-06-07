import { useWallet } from "@fuels/react";
import { useMutation } from "@tanstack/react-query";
import toast from "react-hot-toast";
import { contracts } from "../generated/contract";
import { ContractFactory } from "fuels";
import { NFTContractAbi__factory } from "@/contract-types";
import { AssetIdInput } from "@/contract-types/contracts/NFTContractAbi";
import { createAssetId, createSubId } from "@/utils/assetId";

type CreateNFT = {
  cid: string;
  name: string;
  symbol: string;
  numberOfCopies: number;
};

export const useCreateNFT = () => {
  const { wallet } = useWallet();
  const { bytecode, abi } = contracts["nft-contract"];

  const mutation = useMutation({
    mutationFn: async ({ cid, name, symbol, numberOfCopies }: CreateNFT) => {
      console.log(`wallet.provider.url`, wallet?.provider.url);
      if (!wallet)
        throw new Error(
          `Cannot create NFT if wallet is ${wallet}.  Please connect your wallet.`
        );

      const factory = new ContractFactory(bytecode, abi, wallet);
      const deployedContract = await factory.deployContract({
        configurableConstants: { MAX_SUPPLY: numberOfCopies },
      });

      const contract = NFTContractAbi__factory.connect(
        deployedContract.id,
        wallet
      );

      const constructorCall = contract.functions.constructor({
        Address: { bits: wallet.address.toB256() },
      });

      let contractCalls = [];
      contractCalls.push(constructorCall);
      for (let i = 1; i <= numberOfCopies; ++i) {
        const subId = createSubId(i);
        const assetId: AssetIdInput = createAssetId(subId, deployedContract.id.toB256());
        contractCalls.push(
          contract.functions.set_metadata(assetId, "image", { String: cid })
        );
        contractCalls.push(contract.functions.set_name(assetId, name));
        contractCalls.push(contract.functions.set_symbol(assetId, symbol));
      }
      await contract.multiCall(contractCalls).call();
      return deployedContract.id;
    },
    onSuccess: () => {
        toast.success("NFT successfully created.");
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
