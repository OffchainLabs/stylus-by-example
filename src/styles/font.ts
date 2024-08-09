import { Inter as createInter } from 'next/font/google';
import localFont from 'next/font/local';

export const font_body = localFont({
  display: 'swap',
  src: '../fonts/montserrat.ttf',
  variable: '--font-body',
});

export const font_heading = localFont({
  display: 'swap',
  src: '../fonts/ropa.ttf',
  variable: '--font-heading',
});

export const font_mono = localFont({
  display: 'swap',
  src: '../fonts/monaspace_xenon.ttf',
  variable: '--font-mono',
});
