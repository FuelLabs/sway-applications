import { createConfig } from "./helpers/createConfig";
import { buildContracts } from "./actions/buildContracts";
import { deployContracts } from "./actions/deployContracts";
import { callSetAuthorizer } from "./calls/setAuthorizerCall";
import { Wallet } from "fuels";

const config = createConfig({
  types: {
    // artifacts: "../**/out/debug/**-abi.json",
    artifacts: "../vault/out/debug/**-abi.json",
    output: "./pkg/types/contracts",
  },
  contracts: [
    {
      name: "VAULT_CONTRACT",
      path: "../vault",
    },
  ],
});

const main = async () => {
  const wallet = new Wallet(
    "0x37fa81c84ccd547c30c176b118d5cb892bdb113e8e80141f266519422ef9eefd"
  );
  await buildContracts(config);
  const contractIds = await deployContracts(config, wallet);
  console.log("Contracts Deployed:", contractIds);
  const res = await callSetAuthorizer(contractIds[0].contractId, wallet);
  console.log(res);
};

main().then(() => process.exit).catch(err => console.log(err));