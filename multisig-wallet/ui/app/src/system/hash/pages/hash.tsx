import { BoxCentered, Flex, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { ExecuteHashComponent } from "../components/execution";
import { WeightHashComponent } from "../components/weight";
import { TransferHashComponent } from "../components/transfer";
import { ThresholdHashComponent } from "../components/threshold";
import { RadioGroupComponent } from "../../common/components/radio_group";
import { OptionalCheckBoxComponent } from "../../common/components/optional_data_checkbox";

export function HashPage() {
    const [recipient, setRecipient] = useState("address")
    const [optionalData, setOptionalData] = useState(false)

    return (
        <BoxCentered css={{ marginTop: "3%", width: "30%" }}>
            <Stack>
                <Flex gap="130px" css={{ marginBottom: "$14" }}>
                    <ExecuteHashComponent recipient={recipient} />
                    <TransferHashComponent recipient={recipient} />
                </Flex>

                <Flex gap="130px" css={{ marginBottom: "$10" }}>
                    <WeightHashComponent optionalData={optionalData} />
                    <ThresholdHashComponent optionalData={optionalData} />
                </Flex>

                <OptionalCheckBoxComponent handler={setOptionalData} optionalData={optionalData} />
                <RadioGroupComponent handler={setRecipient} />
            </Stack>
        </BoxCentered>
    );
}
