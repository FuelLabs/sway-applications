import { Box } from "@fuel-ui/react";

import { MOCK_AUCTIONS } from "../../__mocks__/auctions";

import { AuctionPages } from "./AuctionPages";

export default {
    component: AuctionPages,
    title: "Buy/components/AuctionPages",
};

export const Usage = () => {
    return (
        <Box css={{ width: 320 }}>
            <AuctionPages auctions={MOCK_AUCTIONS} />
        </Box>
    );
}
