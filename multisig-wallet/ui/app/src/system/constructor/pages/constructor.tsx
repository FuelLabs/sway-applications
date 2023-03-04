import { Button, toast } from "@fuel-ui/react";

export function ConstructorPage() {
    return (
        <>
            <Button
                color="accent"
                onPress={function noRefCheck(){ toast.error("Unimplemented") }}
                size="md"
                variant="solid"
                css={{ margin: "auto" }}
            >
                Create
            </Button>
        </>
    );
}
