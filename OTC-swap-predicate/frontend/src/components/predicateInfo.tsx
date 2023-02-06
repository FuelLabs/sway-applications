import { FC } from "react";
import { Address, CoinQuantity } from "fuels";
import { ZERO_ADDRESS } from "../utils/constants";
import TokenList from "./tokensList";


type PredicateInfoProps = {
    predicateAddress: Address,
    tokensFound: CoinQuantity[],
    handleTake: () => void,
    handleCancel: () => void,
}


const PredicateInfo: FC<PredicateInfoProps> = ({predicateAddress, tokensFound, handleTake, handleCancel}: PredicateInfoProps) => {
    return (
        <>
            {/* This will only render if predicateAddress is not empty */}
            {predicateAddress !== ZERO_ADDRESS &&
                <>
                <p>To fund this offer, send tokens to :</p>
                <p className="App-address">{predicateAddress.toString()}</p>
                
                {/* Render tokens found belonging to predicate address */}
                <TokenList tokensFound={tokensFound} handleTake={handleTake} handleCancel={handleCancel}/>
                </>
            }
        </>
        )
    }

export default PredicateInfo;
