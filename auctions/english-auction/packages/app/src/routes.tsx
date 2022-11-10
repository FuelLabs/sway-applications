import { Navigate, Route, Routes } from "react-router-dom";

import { Pages } from "./types";

export const routes = (
  <Routes>
    <Route>
      <Route path="*" element={<Navigate to={Pages.home} />} />
    </Route>
  </Routes>
);
