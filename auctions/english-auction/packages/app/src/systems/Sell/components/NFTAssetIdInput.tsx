import { Form, Input } from "@fuel-ui/react";

interface NFTAssetIdInputProps {
    onChange: (key: string, value: string) => void;
    nftAssetIdValue: string;
    objKey: string;
    label: string;
};

export const NFTAssetIdInput = ({ onChange, nftAssetIdValue, objKey, label }: NFTAssetIdInputProps) => {

    return (
        <Form.Control isRequired css={{ minWidth: "100%" }}>
            <Form.Label>{label}</Form.Label>
            <Input>
                <Input.Field
                    id={objKey}
                    onChange={(e) => onChange(objKey, e.target.value)}
                    placeholder="0x000...000"
                    value={nftAssetIdValue}
                />
            </Input>
        </Form.Control>
    );
}
