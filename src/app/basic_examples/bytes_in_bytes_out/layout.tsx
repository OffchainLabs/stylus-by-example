import { Metadata } from "next";

interface LayoutProps {
  children: React.ReactNode;
}

export const metadata: Metadata = {
  title: "Bytes In, Bytes Out • Stylus by Example",
  description:
    "An introduction to Arbitrum Stylus with simple code examples in Rust and WASM",
};

export default function PageLayout({ children }: LayoutProps) {
  return <>{children}</>;
}
