import { render } from "@testing-library/react";
import MerchantPage from "./page";

describe("MerchantPage", () => {
  it("renders without crashing", () => {
    render(<MerchantPage />);
  });
});
