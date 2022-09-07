import { Flex, Card, Button } from "@fuel-ui/react";
import { formatValue } from "../utils/helpers";
import { DECIMAL_PLACES } from "@/config";

import type { EscrowInfoOutput } from "@/types/contracts/EscrowAbi"

interface EscrowInfoProps {
    escrows: EscrowInfoOutput[] | undefined
}

export function EscrowInfo({ escrows }: EscrowInfoProps) {

    return (
        <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
            {(!!escrows && escrows.length > 0)
                ? <>
                    <Card.Header>
                        Buyer Escrows
                    </Card.Header>
                    <Card.Body>
                        <div>{`Arbiter: ${!!escrows[0].arbiter.address.Address ? escrows[0].arbiter.address.Address?.value : escrows[0].arbiter.address.ContractId!.value}`}</div>
                        <div>{`Arbiter Asset: ${escrows[0].arbiter.asset.value}`}</div>
                        <div>{`Arbiter Fee: ${formatValue(escrows[0].arbiter.fee_amount, DECIMAL_PLACES)}`}</div>
                        {escrows[0].assets.map((assetInfo) => {
                            return <>
                                <div>{`Asset Id: ${assetInfo.id.value}`}</div>
                                <div>{`Asset Amount: ${formatValue(assetInfo.amount, DECIMAL_PLACES)}`}</div>
                            </>
                        })}
                        <div>{`Buyer: ${!!escrows[0].buyer.address.Address ? escrows[0].buyer.address.Address?.value : escrows[0].buyer.address.ContractId?.value}`}</div>
                        {/** TODO fix buyer.asset.None/Some is undefined */}
                        <div>{`Buyer Desposit Asset: ${!escrows[0].buyer.asset ? "None" : escrows[0].buyer.asset.values}`}</div>
                        <div>{`Buyer Deposit Amount: ${formatValue(escrows[0].buyer.deposited_amount, DECIMAL_PLACES)}`}</div>
                        <div>{`Seller: ${!!escrows[0].seller.address.Address ? escrows[0].seller.address.Address?.value : escrows[0].seller.address.ContractId?.value}`}</div>
                        <div>{`Deadline: ${escrows[0].deadline.toString()}`}</div>
                        <div>{`Disputed: ${escrows[0].disputed}`}</div>
                        <div>{`State: ${!!escrows[0].state.Pending ? "Pending" : "Completed"}`}</div>
                    </Card.Body>

                    <Card.Footer justify="space-evenly">
                        {/* <Button onPress={() => handleAcceptArbiter(BigInt(0))}>
                        Accept Arbiter
                    </Button>
                    <Button onPress={() => transferToSellerMutation.mutate()}>
                        Transfer To Seller
                    </Button>
                    <Button onPress={() => disputeMutation.mutate()}>
                        Dispute
                    </Button> */}
                    </Card.Footer>

                    <Card.Footer direction="row-reverse" gap="$4">
                        <Button leftIcon="DotsThree">
                            Show all escrows
                        </Button>
                    </Card.Footer>
                </>
                : <>
                    <Card.Header>
                        Buyer Escrows
                    </Card.Header>
                    <Card.Body>
                        Buyer has no Escrows
                    </Card.Body>
                </>
            }

        </Card>
    )
}