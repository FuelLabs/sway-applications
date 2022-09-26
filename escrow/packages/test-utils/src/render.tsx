import "@testing-library/jest-dom";
import type { RenderOptions } from "@testing-library/react";
import { render as rtlRender } from "@testing-library/react";
import type { ReactNode } from "react";
import { MemoryRouter } from "react-router-dom";

import { userEvent } from "./user-event";

export function render(
  ui: React.ReactElement,
  options: RenderOptions = {}
): ReturnType<typeof rtlRender> & { user: ReturnType<typeof userEvent.setup> } {
  const user = userEvent.setup();
  const result = rtlRender(ui, options);
  return { user, ...result };
}

function wrapper(route?: string) {
  return ({ children }: { children: ReactNode }) => (
    <MemoryRouter initialEntries={route ? [route] : []}>
      {children}
    </MemoryRouter>
  );
}

export function renderWithRouter(
  ui: React.ReactElement,
  { route, ...options }: RenderOptions & { route?: string } = { route: "/" }
): ReturnType<typeof rtlRender> & { user: ReturnType<typeof userEvent.setup> } {
  const user = userEvent.setup();
  const result = rtlRender(ui, { ...options, wrapper: wrapper(route) });
  return { user, ...result };
}
