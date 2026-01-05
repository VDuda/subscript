import { render } from "@testing-library/react";
import { Button } from "./button";

describe("Button", () => {
  it("renders without crashing", () => {
    render(<Button appName="test">test</Button>);
  });
});
