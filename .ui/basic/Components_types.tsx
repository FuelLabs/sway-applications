import { useState, useEffect } from "react";
import * as everything from "./contracts";
import * as vars from "./vars.json";
import {
  FunctionFragment,
  FunctionInvocationScope,
  InvokeFunction,
  Wallet,
  WalletLocked,
} from "fuels";

const intermediateJson = vars;

const WALLET_SECRET = intermediateJson.walletSecret;
const CONTRACT_IDS = intermediateJson.contractIds;
const CONTRACT_NAMES = intermediateJson.contractNames;

const wallet = Wallet.fromPrivateKey(WALLET_SECRET);

type ContractsFactories = typeof everything;

type ContractsFactoriesNames = keyof ContractsFactories;

type FunctionsDefinitions = {
  [Property in ContractsFactoriesNames]: {
    [key in keyof ReturnType<ContractsFactories[Property]['connect']>['functions']]: ReturnType<ContractsFactories[Property]['connect']>['functions'][key]
  };
};

type ContractsFactoryFunctionNames<FN extends ContractsFactoriesNames> = keyof FunctionsDefinitions[FN];

type FactoryInvokeFunction<
  FN extends ContractsFactoriesNames,
  FNN extends ContractsFactoryFunctionNames<FN>
> = FunctionsDefinitions[FN][FNN];

const mapp = {
  'u32': Number,
  'u64': bn,
}

function createComponent<
  FN extends ContractsFactoriesNames,
  FNN extends ContractsFactoryFunctionNames<FN>,
>(
  factoryName: FN,
  fName: FNN,
  fragmentValue: FunctionFragment,
  contractFunction: FactoryInvokeFunction<FN, FNN>
) {
  let functionName = fName;
  let inputs = fragmentValue.inputs;
  let outputs = fragmentValue.outputs;

  const factory = everything[factoryName];
  factory

  

  // function determineStates() {
  //   var types = Object.fromEntries(
  //     outputTypes().map((type: any, index: number) => [index, type])
  //   );
  // }

  // function inputTypes() {
  //   return inputs.map((input: any) => input.type);
  // }

  // function outputTypes() {
  //   return outputs.map((output: any) => output.type);
  // }

  // async function getFunction() {
  //   const { value } = await contractFunction().get();
  //   return Number(value);
  // }

  // async function callFunction() {
  //   const { value } = await contractFunction().call();
  //   // console.log(value);
  //   // console.log(Object.getPrototypeOf(value).name);
  // }

  // return (
  //   <div className={functionName}>
  //     <p>{functionName}</p>
  //     <button onClick={callFunction}>{functionName}</button>
  //   </div>
  // );

  return <></>;
}

function Car() {
  const [car, setCar] = useState("ferrari");

  async function change() {
    setCar(car === "ferrari" ? "twingo" : "ferrari");
  }

  return (
    <div className="car">
      <p>{car}</p>
      <button onClick={change}>
        {car === "ferrari" ? "twingo" : "ferrari"}
      </button>
    </div>
  );
}

function GetContracts() {
  let typeImports = Object.getOwnPropertyDescriptors(everything);
  let factories = Object.values(typeImports)
    .filter((value) => value.get !== undefined)
    .map((value) => value.get!());
  let contracts: { [key: string]: Object } = {};
  let components: JSX.Element[] = [];

  for (let i = 0; i < factories.length; i++) {
    const contract = factories[i].connect(CONTRACT_IDS[i], wallet);

    contracts[CONTRACT_NAMES[i]] = contract;

    const fragmentObjects = Object.getOwnPropertyDescriptors(
      contract.interface.fragments
    );
    const fragmentNames = Object.keys(fragmentObjects);
    const fragments = Object.values(fragmentObjects);

    const functionObjects = Object.getOwnPropertyDescriptors(
      contract.functions
    );
    const functions = Object.values(functionObjects);

    for (let j = 0; j < fragmentNames.length - 1; j++) {
      const factoryName = factories[i].name;
      const funcFragment = fragments[j].value!;
      const funcExec = functions[j].value!;
      components.push(createComponent(factoryName, funcFragment.name, funcFragment, funcExec));
    }
  }
  return components;
}

function Components() {
  const components = GetContracts();
  return components;
}

export default Components;
