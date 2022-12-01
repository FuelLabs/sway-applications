import { Link } from "@fuel-ui/react";
import { TransactionResult } from "fuels";
import { BLOCK_EXPLORER_URL } from "~/config";
import { toast } from "@fuel-ui/react";

import { Maybe } from "~/types";

// export const getBlockExplorerLink = (path: string) => {
//     return `${BLOCK_EXPLORER_URL}?providerUrl=${encodeURIComponent(
//         process.env.VITE_FUEL_PROVIDER_URL as string
//     )}`;
// };

interface TxLinkProps {
    id?: string;
};

export function TxLink({ id }: TxLinkProps) {
    return (
        <p>
            <Link isExternal href={`${BLOCK_EXPLORER_URL}/transaction/${id}`}>
                View it on Fuel Explorer
            </Link>
        </p>
    );
}

export const txFeedback = (
    txMsg: string,
    onSuccess?: (data: TransactionResult<any>) => void | Promise<void>
) => {
    return async (data: Maybe<TransactionResult<any>>) => {
        const txLink = <TxLink id={data?.transactionId} />;

        /**
         * Show a toast success message if status.type === 'success'
         */
        if (data?.status.type === "success") {
            await onSuccess?.(data);
            toast.success(
                <>
                    {" "}
                    {txMsg} {txLink}{" "}
                </>,
                { duration: 8000 }
            );
            return;
        }

        /**
         * Show a toast error if status.type !== 'success''
         */
        toast.error(<>Transaction reverted! {txLink}</>);
    };
}