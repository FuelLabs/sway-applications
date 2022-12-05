import { Card, Flex, Heading } from "@fuel-ui/react";

import { getAssetText } from "../utils/text";

interface AssetOutputProps {
    assetId: string;
    assetAmount: string;
    heading: string;
};

export const AssetOutput = ({ assetId, assetAmount, heading }: AssetOutputProps) => {

    return (
        <Card>
            <Card.Header>
                <Heading as="h5">{heading}</Heading>
            </Card.Header>
            <Card.Body>
                <Flex gap="$2">
                    <div>{assetAmount}</div>
                    <div>{getAssetText(false, assetId)}</div>
                </Flex>
            </Card.Body>
        </Card>
    );
};