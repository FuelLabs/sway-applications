import { toast } from "@fuel-ui/react";
import { copy } from "clipboard";
import { QueryClient } from "react-query";

const panicError = (msg: string) => {
  <div>
    Unexpected block execution error
    <br />
    <span className="text-sx text-gray-300">
      <a href="#" onClick={() => copy(msg)}>
        Click here
      </a>{" "}
    </span>
  </div>;
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function handleError(error: any) {
  const msg = error?.message;
  toast.error(msg?.includes("Panic") ? panicError(msg) : msg, {
    duration: 100000000,
    id: msg,
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
