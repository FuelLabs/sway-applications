import { Heading } from "@fuel-ui/react";

interface HeadingInput {
  text: string;
}

export const HeadingComponent = ({ text }: HeadingInput) => {
  return (
    <>
      <Heading
        as="h3"
        css={{
          marginLeft: "auto",
          marginRight: "auto",
          marginBottom: "$10",
          color: "$accent1",
        }}
      >
        {text}
      </Heading>
    </>
  );
};
