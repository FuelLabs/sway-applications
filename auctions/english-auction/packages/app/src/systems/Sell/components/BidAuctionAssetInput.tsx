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
    const id = "nftAssetIdBid";

    const handleAssetChange = (newIsNFT: boolean, assetType: string) => {
        setIsNFT(newIsNFT);
        if (newIsNFT) {
            onChange(id, assetType);
        } else {
            // TODO remove, i dont think this is used, but im fixing something else rn
            onChange("assetIdBid", assetType);
        }
    }

    return (
        <DropdownContainer onChange={handleAssetChange} assets={assets}>
            <Form.Control isRequired>
                <Form.Label>Bid Asset</Form.Label>
                {isNFT && (
                    <NFTAssetIdInput onChange={onChange} label="Bid NFT Asset Id" id={id} nftAssetIdValue={nftAssetIdValue} />
                    // <Form.Control isRequired css={{ minWidth: "100%" }}>
                    //     <Form.Label>Bid NFT Asset Id</Form.Label>
                    //     <Input>
                    //         <Input.Field
                    //             id="nftAssetIdBid"
                    //             onChange={(e) => onChange("nftAssetIdBid", e.target.value)}
                    //             placeholder="0x000...000"
                    //             value={nftAssetIdValue}
                    //         />
                    //     </Input>
                    // </Form.Control>
                )}
            </Form.Control>
        </DropdownContainer >
    );
}