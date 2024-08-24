import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Choose your starter",
  description: "Mint an NFT of a starter Pokemon of your choice using solana blinks",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
