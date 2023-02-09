import { FC } from "react";
import { Address, CoinQuantity } from "fuels";
import { ZERO_ADDRESS } from "../utils/constants";
import TokenList from "./tokensList";
import { Copyable, Stack, Text } from "@fuel-ui/react";


type PredicateInfoProps = {
    predicateAddress: Address,
    tokensFound: CoinQuantity[],
}

const PredicateInfo: FC<PredicateInfoProps> = ({predicateAddress, tokensFound}: PredicateInfoProps) => {


    if (predicateAddress === ZERO_ADDRESS) {
        return null;
    }

    if (tokensFound.length === 0) {

        let addressString = predicateAddress.toString();

        return (
            <Stack css={{ maxW: "600px", textAlign: "center" }}>
                <Text css={{fontSize : 20}}>
                    To fund this offer, send tokens to :
                </Text>
                <Copyable value={addressString}>
                    <p className="App-address">{addressString}</p>
                </Copyable>
                <Text css={{ color: "red", fontSize: "10px"}}>
                    WARNING: Spending / recovery not yet supported by this UI. Use real funds ONLY if you know what you're doing!
                </Text>
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
