import { Route, Routes } from "react-router-dom";
import { ConstructorPage } from "../../constructor/pages";
import { ExecuteTransactionPage } from "../../execute_transaction/pages";
import { HashPage } from "../../hash/pages";
import { ThresholdPage } from "../../threshold/pages";
import { TransferPage } from "../../transfer/pages";
import { UtilsPage } from "../../utils/pages";
import { WeightPage } from "../../weight/pages";

export function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<ConstructorPage />} />
      <Route path="/create" element={<ConstructorPage />} />
      <Route path="/execute" element={<ExecuteTransactionPage />} />
      <Route path="/transfer" element={<TransferPage />} />
      <Route path="/threshold" element={<ThresholdPage />} />
      <Route path="/weight" element={<WeightPage />} />
      <Route path="/hash" element={<HashPage />} />
      <Route path="/utils" element={<UtilsPage />} />
    </Routes>
  );
}
