import { FC } from "react";
import { Address } from "fuels";

type PredicateInfoProps = {
    predicateAddress: string,
}


const PredicateInfo: FC<PredicateInfoProps> = ({predicateAddress}: PredicateInfoProps) => {
    return (
        <>
            {predicateAddress.length > 0 &&
                <>
                <p>To fund this offer, send tokens to :</p>
                <p className="App-address">{Address.fromAddressOrString(predicateAddress).toString()}</p>
                </>
            }
        </>
        )
    }

export default PredicateInfo;
