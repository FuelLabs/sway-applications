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
    if (predicateAddress === ZERO_ADDRESS) {
        return null;
    }

    if (tokensFound.length === 0) {
        return (
            <>
                <p>To fund this offer, send tokens to :</p>
                <p className="App-address">{predicateAddress.toString()}</p>
                <p style={{fontSize: "10px", color:"red"}}> WARNING: Spending / recovery not yet supported by this UI. Use real funds ONLY if you know what you're doing!</p>
            </>
        )
    }

    else {
        return (
            <TokenList tokensFound={tokensFound} handleTake={handleTake} handleCancel={handleCancel}/>
        )
    }
}

export default PredicateInfo;
