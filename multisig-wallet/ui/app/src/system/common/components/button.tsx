import { Button } from "@fuel-ui/react";

interface ButtonInput {
    handler: () => void,
    isConnected: boolean,
    text: string
}

export const ButtonComponent = ({ handler, isConnected, text }: ButtonInput) => {

    return (
        <Button
            color="accent"
            onPress={handler}
            size="lg"
            variant="solid"
            isDisabled={!isConnected}
            css={{ boxShadow: "0px 0px 1px 1px" }}
        >
            {text}
        </Button>
    );
}
