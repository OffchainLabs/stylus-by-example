'use client';
import * as React from 'react';
import { useTheme } from 'next-themes';
import { Sidebar } from '@/components/app/sidebar';
import { Stack } from '@/styled-system/jsx';
import { css } from '@/styled-system/css';

const SIDEBAR_WIDTH = { 'base': '0', 'md': '256px', '2xl': '368px' };

export default function SideNav() {
  const { resolvedTheme } = useTheme();

  if (resolvedTheme === 'dark') {
    return (
      <Stack
        className={css({
          flexShrink: '0',
          w: SIDEBAR_WIDTH,
          h: { base: '0', md: 'full' },
          overflowY: 'auto',
          borderRightWidth: '1px',
          borderColor: 'stone.200',
          background: 'stone.100',
          display: { base: 'none !important', md: 'flex !important' },
          _dark: {
            borderColor: 'stone.950',
            background: 'stone.900',
          },
          scrollbarWidth: 'thin',
          scrollbarColor: 'stone.700 rgba(255,255,255,0)',
          _hover: {
            _scrollbarThumb: {
              display: 'initial',
            },
          },
          _scrollbar: {
            w: '10px',
          },
          _scrollbarThumb: {
            background: 'stone.700',
            borderRadius: '14px',
            display: 'none',
          },
          _scrollbarTrack: {
            background: 'stone.900',
            // display: "none",
          },
        })}
      >
        <Sidebar />
      </Stack>
    );
  }

  return (
    <Stack
      className={css({
        flexShrink: '0',
        w: SIDEBAR_WIDTH,
        h: { base: '0', md: 'full' },
        overflowY: 'auto',
        borderRightWidth: '1px',
        borderColor: 'stone.200',
        background: 'stone.100',
        display: { base: 'none !important', md: 'flex !important' },
        _dark: {
          borderColor: 'stone.950',
          background: 'stone.900',
        },
        scrollbarWidth: 'thin',
        scrollbarColor: 'stone.400 rgba(255,255,255,0)',
        _hover: {
          _scrollbarThumb: {
            display: 'initial',
          },
        },
        _scrollbar: {
          w: '10px',
        },
        _scrollbarThumb: {
          background: 'stone.400',
          borderRadius: '14px',
          display: 'none',
        },
        _scrollbarTrack: {
          background: 'stone.200',
          // display: "none",
        },
      })}
    >
      <Sidebar />
    </Stack>
  );
}
