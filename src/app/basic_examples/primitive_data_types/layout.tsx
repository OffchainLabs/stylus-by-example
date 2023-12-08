import { Metadata } from "next";

interface LayoutProps {
  children: React.ReactNode;
}

export const metadata: Metadata = {
  title: "Primitive Data Types • Stylus by Example",
  description:
    "An introduction to Arbitrum Stylus with simple code examples in Rust and WASM",
};

export default function PageLayout({ children }: LayoutProps) {
  return <>{children}</>;
}
