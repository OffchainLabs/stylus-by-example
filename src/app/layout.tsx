import type { Metadata } from 'next';

import { HStack, Stack } from '@/styled-system/jsx';
import { css } from '@/styled-system/css';
import { container } from '@/styled-system/patterns';
import Navigation from '@/components/app/navigation';

import './globals.css';
import { ThemeProvider } from './theme_provider';
import SideNav from './side_nav';
import { Breadcrumbs } from '@/components/app/breadcrumbs';
import { PageScroll } from '@/components/app/page_scroll';
import { PHProvider } from './providers';
import dynamic from 'next/dynamic';

const PostHogPageView = dynamic(() => import('./PostHogPageView'), {
  ssr: false,
});

export const metadata: Metadata = {
  title: 'Stylus by Example',
  description: 'An introduction to Arbitrum Stylus with simple code examples in Rust and WASM',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html suppressHydrationWarning>
      <PHProvider>
        <body
          className={css({
            overflow: 'hidden',
            h: '100dvh',
            w: '100dvw',
            fontFamily: 'system-ui, Roboto, sans-serif',
          })}
        >
          <PostHogPageView />

          <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
            <Stack
              gap="0"
              h="full"
              w="full"
              overflow="hidden"
              bg="#ffffff"
              flexDir={{ base: 'column-reverse', md: 'column' }}
              _dark={{ bg: '#1b1b1d' }}
            >
              <Navigation />
              <HStack
                className={css({
                  flexGrow: '1',
                  overflow: 'hidden',
                })}
              >
                <SideNav />
                <PageScroll>
                  <div
                    className={container({
                      maxW: '5xl',
                    })}
                  >
                    <Breadcrumbs />
                    {children}
                  </div>
                </PageScroll>
              </HStack>
            </Stack>
          </ThemeProvider>
        </body>
      </PHProvider>
    </html>
  );
}
