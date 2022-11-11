import { Route, Routes } from "react-router-dom";

import { homeRoutes } from "./systems/Home";

export const routes = (
  <Routes>
    <Route>{homeRoutes}</Route>
  </Routes>
);
