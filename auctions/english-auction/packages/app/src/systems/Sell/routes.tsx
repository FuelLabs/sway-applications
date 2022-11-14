import { Route } from "react-router-dom";

import { SellPage } from "./pages";

import { Pages } from "~/types";

export const sellRoutes = <Route path={Pages.sell} element={<SellPage />} />;
