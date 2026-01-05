import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Subscriber View",
  description: "Subscriber dashboard for SubScript",
};

export default function SubscriberLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
