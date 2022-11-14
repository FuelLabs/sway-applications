import { Route, Routes } from "react-router-dom";

import { buyRoutes } from "./systems/Buy";
import { homeRoutes } from "./systems/Home";
import { sellRoutes } from "./systems/Sell";

export const routes = (
  <Routes>
    <Route>{homeRoutes}</Route>
    <Route>{sellRoutes}</Route>
    <Route>{buyRoutes}</Route>
  </Routes>
);
