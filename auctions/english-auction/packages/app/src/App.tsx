import { routes } from "./routes";
import { Providers } from "./systems/Core";

export function App() {
  return <Providers>{routes}</Providers>;
}
