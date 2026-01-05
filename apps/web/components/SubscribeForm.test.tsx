import { render } from "@testing-library/react";
import { SubscribeForm } from "./SubscribeForm";

describe("SubscribeForm", () => {
  it("renders without crashing", () => {
    render(<SubscribeForm />);
  });
});
