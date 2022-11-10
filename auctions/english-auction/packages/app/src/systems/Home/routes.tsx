import { Route } from "react-router-dom";

import { Pages } from "../../types";

import { HomePage } from "./pages";

export const homeRoutes = <Route path={Pages.home} element={<HomePage />} />;
