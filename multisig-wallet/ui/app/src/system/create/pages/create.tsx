import { PleaseConnect } from "../../core/components";

export function CreatePage() {
    if (window.fuel?.isConnected()) {
        return (<>lalalala</>);
    } else {
        return (<PleaseConnect />);
    }
}
