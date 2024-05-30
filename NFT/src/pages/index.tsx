import { FuelLogo } from "@/components/FuelLogo";
import { UploadButton } from "@/components/UploadButton";
import contractId from "@/contract-types/contract-ids.json";

export default function Home() {

  return (
      <div className="flex gap-4 items-center">
        <FuelLogo />
        <h1 className="text-2xl font-semibold ali">Welcome to Fuel</h1>
        <UploadButton />
      </div>
  );
}
