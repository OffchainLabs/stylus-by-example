'use client';
import * as React from 'react';
import { useTheme } from 'next-themes';
import { Sidebar } from '@/components/app/sidebar';
import { Stack } from '@/styled-system/jsx';
import { css } from '@/styled-system/css';

const SIDEBAR_WIDTH = { 'base': '0', 'md': '256px', '2xl': '300px' };

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
          background: '#ffffff',
          display: { base: 'none !important', md: 'flex !important' },
          _dark: {
            borderColor: '#444950',
            background: '#1b1b1d',
          },
          scrollbarWidth: 'thin',
          scrollbarColor: 'stone.700 rgba(255,255,255,0)',
          _hover: {
            _scrollbarThumb: {
              display: 'initial',
            },
          },
          _scrollbar: {
            w: '1px',
          },
          _scrollbarThumb: {
            background: 'stone.700',
            borderRadius: '14px',
            display: 'none',
          },
          _scrollbarTrack: {
            background: '#1b1b1d',
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
        background: '#ffffff',
        display: { base: 'none !important', md: 'flex !important' },
        _dark: {
          borderColor: '#444950',
          background: '#1b1b1d',
        },
        scrollbarWidth: 'thin',
        scrollbarColor: 'stone.400 rgba(255,255,255,0)',
        _hover: {
          _scrollbarThumb: {
            display: 'initial',
          },
        },
        _scrollbar: {
          w: '1px',
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
