import { Wallet } from "fuels";
import type { FC } from "react";
import React, { useState, useEffect } from "react";

import { CounterContractAbi__factory as Factory } from "./counter-contract-types";

const wallet = new Wallet(
  process.env.PRIVATE_KEY as string,
  process.env.FUEL_PROVIDER_URL
);
const contractInstance = Factory.connect(
  process.env.CONTRACT_ID as string,
  wallet
);

const failCatch = (error: Error) => {
  console.error("calling submit failed with:", error);
  return -1n;
};

const loadCounter = async (): Promise<bigint> => {
  try {
    const { value } = await contractInstance.functions.get_counter().call();
    return value;
  } catch (e) {
    return failCatch(e as any);
  }
};

const incrementCounter = async (): Promise<bigint> => {
  await contractInstance.functions.increment().call().catch(failCatch);

  return loadCounter();
};

const decrementCounter = async (): Promise<bigint> => {
  await contractInstance.functions.decrement().call().catch(failCatch);

  return loadCounter();
};

const App: FC = () => {
  const [counterValue, setCounterValue] = useState<bigint>(-10n);
  useEffect(() => {
    loadCounter().then(setCounterValue);
  }, [setCounterValue]);

  const handleIncClick = () => incrementCounter().then(setCounterValue);

  const handleDecClick = () => decrementCounter().then(setCounterValue);
  return (
    <div>
      {counterValue === -1n ? (
        <p>Failed to retrieve counter!</p>
      ) : (
        <p>
          Your counter value is{" "}
          <b>{counterValue === -10n ? "loading" : `${counterValue}`}</b>
        </p>
      )}
      {counterValue >= 0 ? (
        <button onClick={handleIncClick}>More Counts</button>
      ) : null}
      {counterValue > 0 ? (
        <button onClick={handleDecClick}>Less Counts</button>
      ) : null}
    </div>
  );
};

export default App;
