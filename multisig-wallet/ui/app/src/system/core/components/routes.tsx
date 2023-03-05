import { Route, Routes } from "react-router-dom";
import { ConstructorPage } from "../../constructor/pages";
import { ExecuteTransactionPage } from "../../execute_transaction/pages";
import { HashPage } from "../../hash/pages";
import { ThresholdPage } from "../../threshold/pages";
import { TransferPage } from "../../transfer/pages";
import { ViewPage } from "../../view/pages";
import { WeightPage } from "../../weight/pages";

export function AppRoutes() {
    return (
        <>
            <Routes>
              <Route path="/" element={<ConstructorPage />} />
              <Route path="/create" element={<ConstructorPage />} />
              <Route path="/execute" element={<ExecuteTransactionPage />} />
              <Route path="/transfer" element={<TransferPage />} />
              <Route path="/update-threshold" element={<ThresholdPage />} />
              <Route path="/update-weight" element={<WeightPage />} />
              <Route path="/hash" element={<HashPage />} />
              <Route path="/view" element={<ViewPage />} />
            </Routes>
        </>
    );
}