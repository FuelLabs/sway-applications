import { Pagination } from "@fuel-ui/react";

export const AuctionPages = () => {

    return (
        <Pagination pagesCount={10}>
            <Pagination.Prev />
            <Pagination.Items />
            <Pagination.Next />
        </Pagination>
    );
};