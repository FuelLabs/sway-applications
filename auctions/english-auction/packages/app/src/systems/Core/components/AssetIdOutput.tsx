import { Card, Heading } from "@fuel-ui/react";

interface AssetIdOutputProps {
  assetId: string;
  heading: string;
}

export const AssetIdOutput = ({ assetId, heading }: AssetIdOutputProps) => {
  return (
    <Card>
      <Card.Header>
        <Heading as="h5">{heading}</Heading>
      </Card.Header>
      <Card.Body>
        <div>{assetId}</div>
      </Card.Body>
    </Card>
  );
};
