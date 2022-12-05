import { BN } from "fuels";
import { useMutation } from "react-query";
import { useContract } from "~/systems/Core/hooks/useContract";
import { handleError } from "~/systems/Core/utils";
import { txFeedback } from "~/systems/Core/utils/feedback";
import { AuctionAssetInput } from "~/types/contracts/EnglishAuctionAbi";

interface UseBidProps {
    auctionId: BN;
    auctionAsset: AuctionAssetInput;
};

export const useBid = ({ auctionId, auctionAsset }: UseBidProps) => {
    const contract= useContract();

    const mutation = useMutation(
        async () => {
            if (!contract) throw("Contract not connected");

            const { transactionResult } = await contract.functions
                .bid(auctionId, auctionAsset)
                .call();
            
            return transactionResult;
        },
        {
            onSuccess: txFeedback('Auction bid placed successfully', handleSuccess),
            onError: handleError,
        }
    );

    function handleSuccess() {
        console.log("bid success");
    }

    return mutation;
};
