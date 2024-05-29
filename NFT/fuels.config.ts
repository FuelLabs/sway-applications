import { CommandEvent, Commands, defaultConsensusKey, FuelsConfig, createConfig, Wallet, Provider } from 'fuels';
import dotenv from 'dotenv';
import { NODE_URL, IS_PROD } from '@/lib';
import { NFTContractAbi__factory } from '@/contract-types';

dotenv.config({
  path: ['.env.local', '.env'],
});

const fuelCorePort = +(process.env.NEXT_PUBLIC_FUEL_NODE_PORT as string) || 4000;

export default createConfig({
  workspace: './',
  output: IS_PROD ? './production-contract' : './src/contract-types',
  fuelCorePort,
  providerUrl: NODE_URL,
  onSuccess: async (event: CommandEvent) => {
    if (event.type !== Commands.deploy) {
      return;
    }

    const { data: contracts } = event;

    const nftContract = contracts.find((contract) => {
      console.log(`contract`, contract);
      return contract.name === "nftContract";
    });

    if (!nftContract) {
      throw new Error("Contract not found");
    }
    
    const provider = await Provider.create(NODE_URL);
    const privateKey = IS_PROD ?  process.env.WALLET_SECRET! : defaultConsensusKey;
    const wallet = Wallet.fromPrivateKey(privateKey, provider);

    // Call nft contructor to initialize owner
    const contract = NFTContractAbi__factory.connect(nftContract.contractId, wallet);
    try {
      await contract.functions.constructor( { Address: { bits: wallet.address.toB256() }}).call();
      console.log("Owner initialized");
    } catch (err) {
      console.error(err);
    }
  }
});
