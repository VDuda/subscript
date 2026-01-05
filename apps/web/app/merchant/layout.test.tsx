import { render } from "@testing-library/react";
import MerchantLayout from "./layout";

describe("MerchantLayout", () => {
  it("renders without crashing", () => {
    render(<MerchantLayout><div>test</div></MerchantLayout>);
  });
});
