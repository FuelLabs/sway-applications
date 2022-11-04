import { Wallet } from "fuels";
import { VaultAbi__factory } from "../../pkg/types/contracts";


export const callSetAuthorizer = async (contractId: string, wallet: Wallet) => {
    console.log("Calling set_authorizer");
  const contractInstance = VaultAbi__factory.connect(contractId, wallet);

  const transaction = await contractInstance.submitResult.set_authorizer({
    value: "0x5701ea3125b7184420ac5300a18485c3ee374ebe4a817c7e5f6abf50322bf11d",
  });

  return transaction;
};
