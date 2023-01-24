import { Route, Routes } from "react-router-dom";

import { buyRoutes } from "./systems/Buy";
import { sellRoutes } from "./systems/Sell";

export const routes = (
  <Routes>
    <Route>{sellRoutes}</Route>
    <Route>{buyRoutes}</Route>
  </Routes>
);
