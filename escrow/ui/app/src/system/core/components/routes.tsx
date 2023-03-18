import { Route, Routes } from "react-router-dom";
import { CreateEscrow } from "../../create_escrow";
import { ManageEscrow } from "../../manage";

export function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<CreateEscrow />} />
      <Route path="/create-escrow" element={<CreateEscrow />} />
      <Route path="/manage-escrows" element={<ManageEscrow />} />
      {/* <Route path="/deposit" element={<CreateEscrow />} />
      <Route path="/dispute" element={<CreateEscrow />} />
      <Route path="/propose-arbiter" element={<CreateEscrow />} />
      <Route path="/resolve-dispute" element={<CreateEscrow />} />
      <Route path="/return-deposit" element={<CreateEscrow />} />
      <Route path="/take-payment" element={<CreateEscrow />} />
      <Route path="/transfer-to-seller" element={<CreateEscrow />} />
      <Route path="/withdraw-collateral" element={<CreateEscrow />} /> */}
    </Routes>
  );
}
