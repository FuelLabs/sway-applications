import { Route, Navigate } from "react-router-dom";

import ArbiterPage from "./pages/ArbiterPage";
import BuyerPage from "./pages/BuyerPage";
import SellerPage from "./pages/SellerPage";

import { Pages } from "../../types";

export const coreRoutes = (
  <Route>
    <Route path="*" element={<Navigate to={Pages.seller} />} />
    <Route path={Pages.seller} element={<SellerPage />} />
    <Route path={Pages.buyer} element={<BuyerPage />} />
    <Route path={Pages.arbiter} element={<ArbiterPage />} />
  </Route>
);
