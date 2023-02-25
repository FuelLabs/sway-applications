import { Link, toast } from "@fuel-ui/react";
import type { TransactionResult } from "fuels";

import { BLOCK_EXPLORER_URL } from "~/config";
import type { Maybe } from "~/types";

interface TxLinkProps {
  id?: string;
}

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
  onSuccess?: (data: TransactionResult<any>) => void | Promise<void> // eslint-disable-line @typescript-eslint/no-explicit-any
) => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
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
};
