'use client';
import Link from 'next/link';
import { useSelectedLayoutSegments } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { RouteInfo } from '@/data/routes';
import { css } from '@/styled-system/css';

interface SidebarLinksProps {
  routes: RouteInfo[];
}

const ACTIVE_ROUTE_COLOR = { _dark: 'pink.400', _light: 'pink.400' };

export function SidebarLinks({ routes }: SidebarLinksProps) {
  const segments = useSelectedLayoutSegments();
  const ACTIVE_ROUTE = `/${segments.join('/')}`;

  return (
    <>
      {routes?.map(({ route, title }, i) => (
        <Button
          asChild
          key={`${title}-${i}`}
          variant="link"
          className={css({
            w: 'full',
            justifyContent: 'flex-start',
            fontWeight: route === ACTIVE_ROUTE ? '600' : '400',
            fontSize: { 'base': '14px', '2xl': '14px' },
            paddingInline: '16px',
            marginLeft: '20px',
            h: { 'base': '32px', '2xl': '36px' },
            textDecoration: { _hover: 'none' },
            backgroundColor: route === ACTIVE_ROUTE ? '#ffffff0d' : { _hover: '#ffffff0d' },
            color: route === ACTIVE_ROUTE ? ACTIVE_ROUTE_COLOR : {},
          })}
        >
          <Link href={route}>{title}</Link>
        </Button>
      ))}
    </>
  );
}
