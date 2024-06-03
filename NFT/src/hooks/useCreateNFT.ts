import { useWallet } from "@fuels/react";
import { useMutation } from "@tanstack/react-query";
import toast from "react-hot-toast";
import { contracts } from "../generated/contract";
import { AssetId, ContractFactory, arrayify, hash } from "fuels";
import { NFTContractAbi__factory } from "@/contract-types";

type CreateNFT = {
    cid: string,
    name: string,
    symbol: string,
    numberOfCopies: number
};

export const useCreateNFT = () => {
  const { wallet } = useWallet();
  const { bytecode, abi } = contracts["nft-contract"];

  const mutation = useMutation({
    mutationFn: async ({
      cid,
      name,
      symbol,
      numberOfCopies,
    }: CreateNFT) => {
      if (!wallet) throw new Error(`Cannot create NFT if wallet is ${wallet}`);

      const factory = new ContractFactory(bytecode, abi, wallet);
      const deployedContract = await factory.deployContract({
        configurableConstants: { MAX_SUPPLY: numberOfCopies },
      });

      const contract = NFTContractAbi__factory.connect(
        deployedContract.id,
        wallet
      );

      await contract.functions
        .constructor({ Address: { bits: wallet.address.toB256() } })
        .call();

      let contractCalls = []
      for (let i = 1; i <= numberOfCopies; ++i) {
        const assetId: AssetId = {
          bits: hash(
            arrayify(
              Array.from({ length: 32 }, () => i)
                .toString()
                .concat(deployedContract.id.toB256())
            )
          ),
        };
        contractCalls.push(contract.functions.set_metadata(assetId, "image", { String: cid }));
        contractCalls.push(contract.functions.set_name(assetId, name));
        contractCalls.push(contract.functions.set_symbol(assetId, symbol));
      }
      await contract.multiCall(contractCalls).call();
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
