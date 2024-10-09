'use client';
import * as React from 'react';
import { useTheme } from 'next-themes';
import { css } from '@/styled-system/css';

export function SideNavScroll({ children }: { children: React.ReactNode }) {
  const { theme } = useTheme();

  if (theme === 'dark') {
    return (
      <div
        className={css({
          flexGrow: '1',
          h: 'full',
          pt: '6',
          pb: '12',
          px: '4',
          overflowY: 'scroll',
          scrollbarWidth: 'thin',
          scrollbarColor: 'stone.700 stone.900',
          _scrollbar: {
            w: '1px',
          },
          _scrollbarThumb: {
            background: 'stone.700',
            borderRadius: '14px',
          },
          _scrollbarTrack: {
            background: '#1b1b1d',
          },
        })}
      >
        {children}
      </div>
    );
  }

  // Default light mode scroll
  return (
    <div
      className={css({
        flexGrow: '1',
        h: 'full',
        pt: '6',
        pb: '12',
        px: '4',
        overflowY: 'scroll',
        scrollbarWidth: 'thin',
        scrollbarColor: 'stone.400 stone.200',
        _scrollbar: {
          w: '1px',
        },
        _scrollbarThumb: {
          background: 'stone.400',
          borderRadius: '14px',
        },
        _scrollbarTrack: {
          background: 'stone.200',
        },
      })}
    >
      {children}
    </div>
  );
}
