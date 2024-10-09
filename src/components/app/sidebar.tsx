import { gettingStarted, basicExamples, applications } from '@/data/routes';
import { css } from '@/styled-system/css';
import { stack } from '@/styled-system/patterns';
import { Stack } from '@/styled-system/jsx';

import { SidebarLinks } from '@/components/app/sidebar_links';
import { ReactNode } from 'react';
import Link from 'next/link';
import { ChevronLeft } from 'lucide-react';
import Image from 'next/image';

const ICON_SIZE = { 'base': '16px', '2xl': '16px' };

function SidebarHeading({ children }: { children: ReactNode }) {
  return (
    <h2
      className={css({
        fontSize: { 'base': '16px', 'md': '16px', '2xl': '16px' },
        fontWeight: '600',
        mb: '1',
        px: '4',
        fontFamily: 'heading',
      })}
    >
      {children}
    </h2>
  );
}

export function Sidebar() {
  return (
    <>
      <SidebarHeading>
        <Link
          className={css({
            fontSize: { 'base': '14px', 'md': '14px', '2xl': '14px' },
            fontWeight: '400',
            fontFamily: 'heading',
            display: 'flex',
            flexWrap: 'nowrap',
            alignItems: 'center',
            gap: '2',
            whiteSpace: 'nowrap',
            textDecoration: { _hover: 'underline' },
            py: '3',
          })}
          href="https://docs.arbitrum.io/stylus/stylus-overview"
        >
          <ChevronLeft
            className={css({
              color: '#808080',
              h: ICON_SIZE,
              w: ICON_SIZE,
            })}
          />
          <Image alt="Arbitrum logo" width={16} height={16} src="/arbitrum_logo.svg" />
          Back to Arbitrum Docs
        </Link>
      </SidebarHeading>

      <div
        className={stack({
          pt: '6',
          px: '1',
          pb: '24',
          gap: '0',
        })}
      >
        <SidebarHeading>Getting Started</SidebarHeading>
        <Stack gap={1} mb={4}>
          <SidebarLinks routes={gettingStarted} />
        </Stack>
        <SidebarHeading>Basic</SidebarHeading>
        <Stack gap={1} mb={4}>
          <SidebarLinks routes={basicExamples} />
        </Stack>
        <SidebarHeading>Applications</SidebarHeading>
        <Stack gap={1} mb={4}>
          <SidebarLinks routes={applications} />
        </Stack>
      </div>
    </>
  );
}
