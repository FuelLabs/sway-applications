import { ThemeProvider } from "@fuel-ui/react";
import { themes } from "@storybook/theming";
import { mswDecorator, initialize } from "msw-storybook-addon";
import { withRouter } from "storybook-addon-react-router-v6";
import { INITIAL_VIEWPORTS } from "@storybook/addon-viewport";

import theme from "./theme";
import { join } from "path";

export const parameters = {
  actions: {
    argTypesRegex: "^on[A-Z].*",
  },
  controls: {
    matchers: {
      color: /(background|color)$/i,
      date: /Date$/,
    },
  },
  options: {
    storySort: (a, b) => {
      return a[1]?.kind === b[1]?.kind
        ? 0
        : a[1]?.id.localeCompare(b[1]?.id, undefined, { numeric: true });
    },
  },
  darkMode: {
    stylePreview: true,
    dark: {
      ...themes.dark,
      ...theme,
    },
    light: {
      ...themes.light,
      ...theme,
    },
  },
  viewport: {
    viewports: {
      ...INITIAL_VIEWPORTS,
      chromeExtension: {
        name: "Chrome Extension",
        styles: {
          height: "600px",
          width: "350px",
        },
        type: "mobile",
      },
    },
  },
};

export const decorators = [
  mswDecorator,
  withRouter,
  (Story) => (
    <ThemeProvider>
      <Story />
    </ThemeProvider>
  ),
];

initialize({
  onUnhandledRequest: "bypass",
  serviceWorker: {
    url: join(process.env.STORYBOOK_BASE_URL || "", "/mockServiceWorker.js"),
  },
});
