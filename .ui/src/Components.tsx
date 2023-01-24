// @ts-nocheck
import { useState } from "react";
import * as everything from "./contracts";
import * as vars from "./vars.json";
import { bn, Contract, FunctionFragment, Wallet } from "fuels";

const intermediateJson = vars;

const WALLET_SECRET = intermediateJson.walletSecret;
const CONTRACT_IDS = intermediateJson.contractIds;

const wallet = Wallet.fromPrivateKey(WALLET_SECRET);

const getInstantiable = (type: string) => {
  switch (type) {
    case "u8":
    case "u16":
    case "u32":
      return {
        type: "number",
        create: Number,
      };
    case "u64":
      return {
        type: "number",
        create: bn,
      };
    case "bool":
      return {
        type: "bool",
        create: Boolean,
      };
    case "b512":
    case "b256":
    case "raw untyped ptr":
      return {
        type: "text",
        create: String,
      };
    default:
      return {
        type: "string",
        create: String,
      };
  }
};

function CreateComponent(contract: Contract, fnFragment: FunctionFragment) {
  const [value, setValue] = useState<{
    [key: string]: string;
  }>({});
  let functionName = fnFragment.name;
  let inputs = fnFragment.inputs;
  let paramsLength = fnFragment.inputs.length;

  async function getFunction(params: any[]) {
    // const { value } = await contract["functions"][functionName](...params).get();
  }

  async function callFunction(params: any[]) {
    // const { value } = await contract["functions"][functionName](...params).call();
  }

  const inputInstances = inputs.map((input) => getInstantiable(input.type));

  const formId = `ContractForm${functionName}`;

  function getParams() {
    const params = inputInstances.map((inputInst, index) => {
      const input = document.querySelector<HTMLInputElement>(
        `#${formId} [name="${index}"]`
      )!;
      return getInstantiable(inputInst.type).create(input.value);
    });
    return params;
  }

  const handleClick = (type: "call" | "get") => async () => {
    try {
      const params = getParams();
      console.log(params);
      const { value } = await contract["functions"]
        [functionName](...params)
        [type]();
      setValue({
        ...value,
        [formId]: String(value),
      });
    } catch (err) {
      setValue({
        ...value,
        [formId]: `error: ${JSON.stringify(err)}`,
      });
    }
  };

  return (
    <div className={functionName}>
      <p>{functionName}</p>
      <form
        id={formId}
        onSubmit={async (e) => {
          e.preventDefault();
        }}
      >
        {inputInstances.map((i, index) => (
          <input type={i.type} name={String(index)} />
        ))}
        <button name="get" onClick={handleClick("get")} type="button">
          Get
        </button>
        <button name="submit" onClick={handleClick("call")} type="button">
          Call
        </button>
      </form>
      <textarea value={value[formId] || ""}></textarea>
    </div>
  );
}

function GetContracts() {
  let typeImports = Object.getOwnPropertyDescriptors(everything);
  let factories = Object.values(typeImports)
    .filter((value) => value.get !== undefined)
    .map((value) => value.get!());
  let components: JSX.Element[] = [];

  console.log(wallet.privateKey);
  for (let i = 0; i < factories.length; i++) {
    const contract = factories[i].connect(CONTRACT_IDS[i], wallet);

    wallet.getBalances().then((balances) => {
      console.log(balances);
    });

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
      components.push(CreateComponent(contract, fragments[j].value));
    }
  }
  return components;
}

function Components() {
  const components = GetContracts();
  return components;
}

export default Components;
