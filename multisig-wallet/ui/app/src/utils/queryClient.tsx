import { toast } from "@fuel-ui/react";
import { QueryClient } from "@tanstack/react-query";

export const panicError = (msg: string) => {
    <div>Unexpected block execution error</div>;
};

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
            retry: false,
            refetchOnWindowFocus: false,
            structuralSharing: false,
        },
        mutations: {
            onError: handleError,
        },
    },
});
