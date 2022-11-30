import { Flex, Form, Input } from "@fuel-ui/react";
import { CoinQuantity, DECIMAL_UNITS } from "fuels";
import { useState } from "react"

import { DropdownContainer } from "./DropdownContainer";

interface BidAuctionAssetInputProps {
    nftAssetIdValue: string;
    onChange: (id: string, val: string) => void;
    assets: CoinQuantity[];
}

export const BidAuctionAssetInput = ({ onChange, nftAssetIdValue, assets }: BidAuctionAssetInputProps) => {
    const [isNFT, setIsNFT] = useState(false);

    const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
        setIsNFT(newIsNFT);
        if (newIsNFT) {
            onChange("nftAssetIdBid", assetType);
        } else {
            onChange("assetIdBid", assetType);
        }
    }

    return (
        <DropdownContainer onChange={handleAssetChange} assets={assets}>
            <Form.Control isRequired>
                <Form.Label>Bid Asset</Form.Label>
                {isNFT ? (
                    <Form.Control isRequired css={{ minWidth: "100%" }}>
                        <Form.Label>Bid NFT Asset Id</Form.Label>
                        <Input>
                            <Input.Field
                                id="nftAssetIdBid"
                                onChange={(e) => onChange("nftAssetIdBid", e.target.value)}
                                placeholder="0x000...000"
                                value={nftAssetIdValue}
                            />
                        </Input>
                    </Form.Control>
                ) : (
                    <></>
                )}
            </Form.Control>
        </DropdownContainer >
    );
}