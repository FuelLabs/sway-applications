import { FuelLogo } from "@/components/FuelLogo";

// const contractId =
//   CURRENT_ENVIRONMENT === "local"
//     ? contractIds.testContract
//     : (process.env.NEXT_PUBLIC_TESTNET_CONTRACT_ID as string); // Testnet Contract ID

export default function Home() {
  // const { wallet, walletBalance, refreshWalletBalance } = useActiveWallet();
  // const [contract, setContract] = useState<TestContractAbi>();
  // const [counter, setCounter] = useState<number>();

  /**
   * useAsync is a wrapper around useEffect that allows us to run asynchronous code
   * See: https://github.com/streamich/react-use/blob/master/docs/useAsync.md
   */
  // useAsync(async () => {
  //   if (hasContract && wallet) {
  //     const testContract = TestContractAbi__factory.connect(contractId, wallet);
  //     setContract(testContract);
  //     const { value } = await testContract.functions.get_count().get();
  //     setCounter(value.toNumber());
  //   }
  // }, [wallet]);

  // // eslint-disable-next-line consistent-return
  // const onIncrementPressed = async () => {
  //   if (!contract) {
  //     return toast.error("Contract not loaded");
  //   }

  //   if (walletBalance?.eq(0)) {
  //     return toast.error(
  //       "Your wallet does not have enough funds. Please click the 'Top-up Wallet' button in the top right corner, or use the local faucet.",
  //     );
  //   }

  //   const { value } = await contract.functions.increment_counter(bn(1)).call();
  //   setCounter(value.toNumber());

  //   await refreshWalletBalance?.();
  // };

  return (
      <div className="flex gap-4 items-center">
        <FuelLogo />
        <h1 className="text-2xl font-semibold ali">Welcome to Fuel</h1>
      </div>
  );
}
