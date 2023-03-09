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
                    <ExecuteHashComponent optionalData={optionalData} recipient={recipient} />
                    <WeightHashComponent optionalData={optionalData} />
                </Flex>

                <Flex gap="130px" css={{ marginBottom: "$10" }}>
                    <TransferHashComponent optionalData={optionalData} recipient={recipient} />
                    <ThresholdHashComponent optionalData={optionalData} />
                </Flex>

                <OptionalCheckBoxComponent setOptionalData={setOptionalData} optionalData={optionalData} />
                <RadioGroupComponent setRecipient={setRecipient} />
            </Stack>
        </BoxCentered>
    );
}
