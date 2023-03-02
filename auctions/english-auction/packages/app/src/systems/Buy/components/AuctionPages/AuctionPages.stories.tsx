import { Box } from "@fuel-ui/react";

import { AuctionPages } from "./AuctionPages";

export default {
    component: AuctionPages,
    title: "Buy/components/AuctionPages",
};

export const Usage = () => {
    return (
        <Box css={{ width: 320 }}>
            <AuctionPages />
        </Box>
    );
}
