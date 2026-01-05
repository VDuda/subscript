import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Merchant View",
  description: "Merchant dashboard for SubScript",
};

export default function MerchantLayout({
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
