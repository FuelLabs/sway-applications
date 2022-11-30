import { Flex, Form, Input } from "@fuel-ui/react";
import { DECIMAL_UNITS } from "fuels";
import { useState } from "react"

import { AuctionAssetDropdown } from "./AuctionAssetDropdown";

interface BidAuctionAssetInputProps {
    assetIdValue: string;
    onChange: (id: string, val: string) => void;
}

export const BidAuctionAssetInput = ({ onChange, assetIdValue }: BidAuctionAssetInputProps) => {
    const [isNFT, setIsNFT] = useState(false);

    const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
        setIsNFT(newIsNFT);
        onChange("assetIdBid", assetType);
    }

    return (
        <Flex>
            <Flex grow={2}>
                <Form.Control isRequired>
                    <Form.Label>Bid Asset</Form.Label>
                    {isNFT ? (
                        <Form.Control isRequired css={{ minWidth: "100%" }}>
                            <Form.Label>Bid NFT Asset Id</Form.Label>
                            <Input>
                                <Input.Field
                                    id="assetIdBid"
                                    onChange={(e) => onChange("assetIdBid", e.target.value)}
                                    placeholder="0x000...000"
                                    value={assetIdValue}
                                />
                            </Input>
                        </Form.Control>
                    ) : (
                        <></>
                    )}
                </Form.Control>
            </Flex>
            <Flex align="start" css={{ marginTop: "$9" }}>
                <AuctionAssetDropdown onChange={handleAssetChange} />
            </Flex>
        </Flex>
    );
}