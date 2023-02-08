import { FC } from "react";
import { Address, CoinQuantity } from "fuels";
import { ZERO_ADDRESS } from "../utils/constants";
import TokenList from "./tokensList";
import { Stack } from "@fuel-ui/react";


type PredicateInfoProps = {
    predicateAddress: Address,
    tokensFound: CoinQuantity[],
}

const PredicateInfo: FC<PredicateInfoProps> = ({predicateAddress, tokensFound}: PredicateInfoProps) => {


    if (predicateAddress === ZERO_ADDRESS) {
        return null;
    }

    if (tokensFound.length === 0) {

        return (
            <Stack css={{ maxW: "600px", textAlign: "center" }}>
                <p>To fund this offer, send tokens to :</p>
                <p className="App-address">{predicateAddress.toString()}</p>
                <p style={{fontSize: "10px", color:"red"}}> WARNING: Spending / recovery not yet supported by this UI. Use real funds ONLY if you know what you're doing!</p>
            </Stack>
        )
    }

    else {
        return (
            <TokenList tokensFound={tokensFound}/>
        )
    }
}

export default PredicateInfo;
