'use client';

import { HStack } from '@/styled-system/jsx';
import { useSelectedLayoutSegments } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { css } from '@/styled-system/css';
import Link from 'next/link';
import { HomeIcon, ChevronRightIcon } from '@radix-ui/react-icons';

const FONT_SIZE = { 'base': '14px', '2xl': '16px' };
const ICON_SIZE = { 'base': '16px', '2xl': '16px' };
const TEXT_COLOR = {
  _dark: 'stone.400',
  _light: 'stone.500',
  _hover: 'pink.400',
};
const ACTIVE_ROUTE_COLOR = { _dark: 'white', _light: 'black' };
const SEPARATOR_COLOR = { _dark: 'white', _light: 'black' };

function HomeButton() {
  return (
    <Button
      asChild
      variant="link"
      className={css({
        p: '0',
        fontSize: FONT_SIZE,
        color: TEXT_COLOR,
        h: { 'base': '32px', '2xl': '36px' },
      })}
    >
      <Link href="/">
        <HomeIcon
          className={css({
            color: SEPARATOR_COLOR,
            h: ICON_SIZE,
            w: ICON_SIZE,
          })}
        />
        Home
      </Link>
    </Button>
  );
}

export function Breadcrumbs() {
  const segments = useSelectedLayoutSegments();
  const LAST_SEGMENT = segments[segments.length - 1];

  return (
    <HStack fontFamily="mono" fontSize={FONT_SIZE} mb={6} flexWrap="wrap" gap="2" rowGap="0">
      <HomeButton />
      {segments.map((segment, index) => (
        <HStack
          className={css({
            whiteSpace: 'nowrap',
            gap: '2',
            justifyContent: 'flex-start',
          })}
          key={`${segment}-${index}`}
        >
          <ChevronRightIcon
            className={css({
              color: SEPARATOR_COLOR,
              h: ICON_SIZE,
              w: ICON_SIZE,
            })}
          />

          <Button
            asChild
            variant="link"
            className={css({
              p: '0',
              m: '0',
              fontSize: FONT_SIZE,
              h: { 'base': '20px', '2xl': '24px' },
              color: segment === LAST_SEGMENT ? ACTIVE_ROUTE_COLOR : TEXT_COLOR,
              textTransform: 'capitalize',
            })}
          >
            <Link href={`/${segments.slice(0, index + 1).join('/')}`}>
              {segment.split('_').join(' ')}
            </Link>
          </Button>
        </HStack>
      ))}
    </HStack>
  );
}
