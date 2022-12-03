import { Flex, Form, Input } from "@fuel-ui/react";
import { CoinQuantity, DECIMAL_UNITS } from "fuels";
import { useState } from "react"

import { DropdownContainer } from "./DropdownContainer";
import { NFTAssetIdInput } from "./NFTAssetIdInput";

interface BidAuctionAssetInputProps {
    nftAssetIdValue: string;
    onChange: (id: string, val: string) => void;
    assets: CoinQuantity[];
}

export const BidAuctionAssetInput = ({ onChange, nftAssetIdValue, assets }: BidAuctionAssetInputProps) => {
    const [isNFT, setIsNFT] = useState(false);

    const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
        setIsNFT(newIsNFT);
        if (!newIsNFT) {
            onChange("assetIdBid", assetType);
        }
    }

    return (
        <DropdownContainer onChange={handleAssetChange} assets={assets}>
            <Form.Control isRequired>
                <Form.Label>Bid Asset</Form.Label>
                {isNFT && (
                    <NFTAssetIdInput
                        onChange={onChange}
                        label="Bid NFT Asset Id"
                        objKey="nftAssetIdBid"
                        nftAssetIdValue={nftAssetIdValue}
                    />
                )}
            </Form.Control>
        </DropdownContainer >
    );
}