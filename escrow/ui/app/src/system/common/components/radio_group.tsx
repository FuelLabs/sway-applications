import { Heading, RadioGroup } from "@fuel-ui/react";

interface ComponentInput {
  handler: (recipient: string) => void;
  text: string;
}

export function RadioGroupComponent({ handler, text }: ComponentInput) {
  return (
    <>
      <Heading
        as="h4"
        css={{
          marginLeft: "auto",
          marginRight: "auto",
          marginTop: "$8",
          color: "$blackA12",
        }}
      >
        {text} Type
      </Heading>

      <RadioGroup
        defaultValue="address"
        direction="row"
        css={{
          margin: "auto",
          ".fuel_form--label": { color: "$blackA12", fontWeight: "$semibold" },
        }}
      >
        <RadioGroup.Item
          onClick={() => handler("address")}
          label="Address"
          value="address"
        />
        <RadioGroup.Item
          onClick={() => handler("contract")}
          label="Contract"
          value="contract"
        />
      </RadioGroup>
    </>
  );
}
