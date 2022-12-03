import { Form, Input } from "@fuel-ui/react";

interface NFTAssetIdInputProps {
    onChange: (key: string, value: string) => void;
    nftAssetIdValue: string;
    id: string;
    label: string;
};

export const NFTAssetIdInput = ({ onChange, nftAssetIdValue, id, label }: NFTAssetIdInputProps) => {

    return (
        <Form.Control isRequired css={{ minWidth: "100%" }}>
            <Form.Label>{label}</Form.Label>
            <Input>
                <Input.Field
                    id={id}
                    onChange={(e) => onChange(id, e.target.value)}
                    placeholder="0x000...000"
                    value={nftAssetIdValue}
                />
            </Input>
        </Form.Control>
    );
}
