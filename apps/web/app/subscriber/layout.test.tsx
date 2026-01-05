import { render } from "@testing-library/react";
import SubscriberLayout from "./layout";

describe("SubscriberLayout", () => {
  it("renders without crashing", () => {
    render(<SubscriberLayout><div>test</div></SubscriberLayout>);
  });
});
