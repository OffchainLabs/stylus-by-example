'use client';

import * as React from 'react';
import { MoonIcon, SunIcon } from '@radix-ui/react-icons';
import { useTheme } from 'next-themes';

import { Button } from '@/components/ui/button';
import { css } from '@/styled-system/css';

export function ModeToggle() {
  const { theme, setTheme } = useTheme();

  const toggleTheme = () => {
    if (theme === 'dark') {
      setTheme('light');
    } else {
      setTheme('dark');
    }
  };

  return (
    <Button
      variant="link"
      className={css({
        w: '36px',
        h: '36px',
        px: '0',
        userSelect: 'none',
        flexDirection: 'column',
        justifyContent: 'center',
        rounded: 'md',
        color: { _light: 'pink.400' },
        bgColor: { _light: 'pink.300' },
        bgGradient: 'to-b',
        gradientFromAlpha: 'muted/50',
        gradientToAlpha: 'muted',
        p: '2',
        textDecoration: 'none',
        outline: 'none',
        borderStyle: 'revert-layer',
        borderColor: { _light: 'pink.300', _dark: 'stone.800' },
        borderWidth: '1px',
        _focus: {
          shadow: 'md',
        },
      })}
      onClick={toggleTheme}
    >
      <SunIcon
        className={css({
          h: '24px',
          w: '24px',
          display: 'inline',
          _dark: { display: 'none' },
        })}
      />
      <MoonIcon
        className={css({
          h: '24px',
          w: '24px',
          display: 'none',
          _dark: { display: 'inline' },
        })}
      />
    </Button>
  );
}
