import { Route } from "react-router-dom";

import { BuyPage } from "./pages";

import { Pages } from "~/types";

export const buyRoutes = <Route path={Pages.buy} element={<BuyPage />} />;
