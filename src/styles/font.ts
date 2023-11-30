import { Inter as createInter } from "next/font/google";
import localFont from "next/font/local";

export const font_sans = createInter({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-sans",
});

export const font_mono = localFont({
  display: "swap",
  src: "../fonts/MonaSpaceXenon.ttf",
  variable: "--font-mono",
});
