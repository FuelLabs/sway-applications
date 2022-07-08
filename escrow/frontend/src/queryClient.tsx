import toast from "react-hot-toast";
import { QueryClient } from "react-query";

const panicError = () => {
  <div>
    Unexpected block execution error
    <br />
  </div>;
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function handleError(error: any) {
  const msg = error.message;
  toast.error(msg.includes("Panic") ? panicError() : msg, {
    duration: 100000000,
  });
}

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      onError: handleError,
      // These two are annoying during development
      retry: false,
      refetchOnWindowFocus: false,
      // This is disabled because it causes a bug with arrays with named keys
      // For example, if a query returns: [BN, BN, a: BN, b: BN]
      // with this option on it will be cached as: [BN, BN]
      // and break our code
      structuralSharing: false,
    },
    mutations: {
      onError: handleError,
    },
  },
});
