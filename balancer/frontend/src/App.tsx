import { Wallet } from "fuels";
import { useEffect, useState } from "react";
import "./App.css";
// Import the contract factory from the generated folder
// from the fuelchain command
import { VaultAbi__factory } from "./contracts";
import CreatePool from "./modules/createPool";
import { MainContainer } from "./styles/styles";

export const tokens: any = {
  token1: {
    name: "token1",
    address:
      "0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token2: {
    name: "token2",
    address:
      "0x34e50d5b80a0391081fb756ba16c499d6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token3: {
    name: "token3",
    address:
      "0x34e50d5b80a0391081fc756ba16c499d6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token4: {
    name: "token4",
    address:
      "0x34e50d5b80a0391081fd756ba16c499d6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token5: {
    name: "token5",
    address:
      "0x34e50d5b80a0391081fe756ba16c499a6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token6: {
    name: "token6",
    address:
      "0x34e50d5b80a0391081fe756ba16c499b6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token7: {
    name: "token7",
    address:
      "0x34e50d5b80a0391081fe756ba16c499c6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token8: {
    name: "token8",
    address:
      "0x34e50d5b80a0391081fe756ba16c499d6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token9: {
    name: "token9",
    address:
      "0x34e50d5b80a0391081fe756ba16c499e6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token10: {
    name: "token10",
    address:
      "0x34e50d5b80a0391081fe756ba16c4f9d6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token11: {
    name: "token11",
    address:
      "0x34e50d5b80a0391081fe756ba16c4g9d6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
  token12: {
    name: "token12",
    address:
      "0x34e50d5b80a0391081fe756ba16ch99d6e3ca990f0e3a62f8417e54092d24934",
    value: "",
  },
};

// Se the Wallet Secret used on the chainConfig.json
// this enables us to have a account with initial balance
const WALLET_SECRET =
  "0x976e5c3fa620092c718d852ca703b6da9e3075b9f2ecb8ed42d9f746bf26aafb";
// The address of the contract deployed to our local node
// the contract id is output right after the forc deploy command
// Ex.: Contract id: 0xa326e3472fd4abc417ba43e369f59ea44f8325d42ba6cf71ec4b58123fd8668a
// const CONTRACT_ID = "0xa326e3472fd4abc417ba43e369f59ea44f8325d42ba6cf71ec4b58123fd8668a"
const CONTRACT_ID =
  "0x701325ef42288797d798f5c6f5ca297bb610017a89b71b66225050eb247bc782";
// Create a Wallet from given secretKey in this case
// The one we configured at the chainConfig.json
const wallet = new Wallet(WALLET_SECRET);
// Connects out Contract instance to the deployed contract
// address using the given wallet.
const contract = VaultAbi__factory.connect(CONTRACT_ID, wallet);

function App() {
  const [poolId, setPoolId] = useState<any | null>(null);
  const [formValues, setFormValues] = useState<any>([
    Object.values(tokens)[0],
    Object.values(tokens)[1],
  ]);

  async function register_pool() {
    // Creates a transactions to call the increment function passing the amount
    // we want to increment, because it creates a TX and updates the contract state
    // this requires the wallet to have enough coins to cover the costs and also
    // to sign the Transaction\

    let pool_id =
      "0x34e50d5b80a0391081fa746ba17c499d8e3ca990f1e3a62f8417e54092d24934";

    let lol = await contract.functions.register_pool(poolId).call();
    console.log(lol);

    // let token1: any =
    //   "0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934";
    // let token2: any =
    //   "0x34e50d5b80a0391081fb756ba16c499d6e3ca990f0e3a62f8417e54092d24934";
    // let token3: any =
    //   "0x34e50d5b80a0391081fc756ba16c499d6e3ca990f0e3a62f8417e54092d24934";
    // let token4: any =
    //   "0x34e50d5b80a0391081fd756ba16c499d6e3ca990f0e3a62f8417e54092d24934";
    // let token5: any =
    //   "0x34e50d5b80a0391081fe756ba16c499d6e3ca990f0e3a62f8417e54092d24934";
    // let token6: any =
    //   "0x34e50d5b80a0391081ff756ba16c499d6e3ca990f0e3a62f8417e54092d24934";
    // let token7: any =
    //   "0x0000000000000000000000000000000000000000000000000000000000000000";
    // let token8: any =
    //   "0x0000000000000000000000000000000000000000000000000000000000000000";
    // let token9: any = "0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934";

    const tokenListSize = 8;
    // create dummy array of length
    const arr = Array.from(Array(tokenListSize).keys());
    let counter = 0;
    const tokenAddressList: any = arr.map((_, index) => {
      if (formValues[index]?.name) {
        counter++;
        return formValues[index]?.address;
      } else {
        return "0x0000000000000000000000000000000000000000000000000000000000000000";
      }
    });

    // if (counter < 2) {
    //   alert("please enter atleast 1 token pair data");
    //   return;
    // }

    const specializationNo = counter > 2 ? 2 : 0;

    // console.log("lol", tokenAddressList);
    // console.log({ specializationNo });

    let am1: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";
    let am2: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";
    let am3: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";
    let am4: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";
    let am5: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";
    let am6: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";
    let am7: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";
    let am8: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";
    let am9: any =
      "0x0000000000000000000000000000000000000000000000000000000000000000";

    // let tokens: any = [
    //   token1,
    //   token2,
    //   token3,
    //   token4,
    //   token5,
    //   token6,
    //   token7,
    //   token8,
    // ];
    let ams: any = [am1, am2, am3, am4, am5, am6, am7, am8];

    // console.log(poolId);
    let lol2 = await contract.functions
      .register_tokens(poolId, specializationNo, tokenAddressList, ams)
      .call();
    console.log(lol2);
  }

  // async function register_toknes() {
  //   // Creates a transactions to call the increment function passing the amount
  //   // we want to increment, because it creates a TX and updates the contract state
  //   // this requires the wallet to have enough coins to cover the costs and also
  //   // to sign the Transaction\
  //   let pool_id = "0x34e50d5b80a0391081fa746ba17c499d8e3ca990f1e3a62f8417e54092d24934";
  //   let token1: any = "0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934";
  //   let token2: any = "0x7e6d6d8a583394cc0385b3cc7fbfc157aedacb0ab7808bb53f71943480b9659e";
  //   let am1: any = "0x0000000000000000000000000000000000000000000000000000000000000000";
  //   let am2: any = "0x0000000000000000000000000000000000000000000000000000000000000000";

  //   let lol2 = await contract.functions.register_tokens(poolId, [token1, token2], [am1, am2]).call();
  //   console.log(lol2);
  // }

  useEffect(() => {
    console.log(formValues);
  }, [formValues]);

  return (
    <MainContainer>
      {/* <button onClick={register_toknes}>register tokens</button> */}
      <CreatePool
        formValues={formValues}
        setFormValues={setFormValues}
        tokens={tokens}
        poolId={poolId}
        setPoolId={setPoolId}
        registerPool={register_pool}
      />
    </MainContainer>
  );
}

export default App;
