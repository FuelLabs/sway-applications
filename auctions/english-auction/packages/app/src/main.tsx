import "@fontsource/inter/variable.css";
import "@fontsource/raleway/variable.css";
import "@fuel-ui/css";

import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router-dom";

import { App } from "./App";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <App />
  </BrowserRouter>
);
