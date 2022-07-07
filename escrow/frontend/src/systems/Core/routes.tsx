import { Route } from "react-router-dom";

import EscrowPage from "./pages/Escrow";

export const coreRoutes = <Route path="*" element={<EscrowPage />} />;
