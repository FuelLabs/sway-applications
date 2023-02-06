import { FC } from "react";
import { Address } from "fuels";
import { ZERO_ADDRESS } from "../utils/constants";

type PredicateInfoProps = {
    predicateAddress: Address,
}


const PredicateInfo: FC<PredicateInfoProps> = ({predicateAddress}: PredicateInfoProps) => {
    return (
        <>
            {/* This will only render if predicateAddress is not empty */}
            {predicateAddress !== ZERO_ADDRESS &&
                <>
                <p>To fund this offer, send tokens to :</p>
                <p className="App-address">{predicateAddress.toString()}</p>
                </>
            }
        </>
        )
    }

export default PredicateInfo;
