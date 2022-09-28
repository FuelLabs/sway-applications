import { Routes } from "react-router-dom";

import { coreRoutes } from "./systems/Core";
import { AppContextProvider } from "./systems/Core/context/AppContext";

export const routes = (
  <AppContextProvider>
    <Routes>{coreRoutes}</Routes>
  </AppContextProvider>
);
