import { Form, Input } from "@fuel-ui/react";

interface NFTAssetIdInputProps {
    onChange: (key: string, value: string) => void;
    nftAssetIdValue: string;
    key: string;
    label: string;
};

export const NFTAssetIdInput = ({ onChange, nftAssetIdValue, key, label }: NFTAssetIdInputProps) => {

    return (
        <Form.Control isRequired css={{ minWidth: "100%" }}>
            <Form.Label>{label}</Form.Label>
            <Input>
                <Input.Field
                    id={key}
                    onChange={(e) => onChange(key, e.target.value)}
                    placeholder="0x000...000"
                    value={nftAssetIdValue}
                />
            </Input>
        </Form.Control>
    );
}
