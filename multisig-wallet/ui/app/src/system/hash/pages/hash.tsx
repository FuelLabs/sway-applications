import { BoxCentered, Flex, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { ExecuteHashComponent, ThresholdHashComponent, TransferHashComponent, WeightHashComponent } from "../components";
import { RadioGroupComponent } from "../../common/components";

export function HashPage() {
    const [recipient, setRecipient] = useState("address")

    return (
        <BoxCentered css={{ marginTop: "3%", width: "30%" }}>
            <Stack>
                <Flex gap="130px" css={{ marginBottom: "$8" }}>
                    <ExecuteHashComponent recipient={recipient} />
                    <TransferHashComponent recipient={recipient} />
                </Flex>

                <Flex gap="130px" css={{ marginBottom: "$5" }}>
                    <WeightHashComponent />
                    <ThresholdHashComponent />
                </Flex>

                <RadioGroupComponent handler={setRecipient} />
            </Stack>
        </BoxCentered>
    );
}
