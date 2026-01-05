import { render } from "@testing-library/react";
import SubscriberPage from "./page";

describe("SubscriberPage", () => {
  it("renders without crashing", () => {
    render(<SubscriberPage />);
  });
});
