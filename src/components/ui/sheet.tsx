'use client';

import * as React from 'react';
import * as SheetPrimitive from '@radix-ui/react-dialog';
import { X } from 'lucide-react';
import { createStyleContext } from '@shadow-panda/style-context';
import { styled } from '@/styled-system/jsx';
import { css } from '@/styled-system/css';
import { sheet, icon } from '@/styled-system/recipes';

const { withProvider, withContext } = createStyleContext(sheet);

const ContentClose = withContext(styled(SheetPrimitive.Close), 'contentClose');

const Content = React.forwardRef<
  React.ElementRef<typeof SheetPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof SheetPrimitive.Content>
>(({ className, children, ...props }, ref) => (
  <SheetPortal>
    <SheetOverlay />
    <SheetPrimitive.Content ref={ref} className={className} {...props}>
      {children}
      <ContentClose
        className={css({
          bottom: '4',
          top: 'unset',
          h: '32px',
          w: '32px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
        })}
      >
        <X className={icon()} />
        <span className={css({ srOnly: true })}>Close</span>
      </ContentClose>
    </SheetPrimitive.Content>
  </SheetPortal>
));
Content.displayName = SheetPrimitive.Content.displayName;

export const Sheet = withProvider(styled(SheetPrimitive.Root), 'root');
export const SheetTrigger = withContext(styled(SheetPrimitive.Trigger), 'trigger');
export const SheetClose = withContext(styled(SheetPrimitive.Close), 'close');
export const SheetPortal = withContext(styled(SheetPrimitive.Portal), 'portal');
export const SheetOverlay = withContext(styled(SheetPrimitive.Overlay), 'overlay');
export const SheetContent = withContext(styled(Content), 'content');
export const SheetHeader = withContext(styled('div'), 'header');
export const SheetFooter = withContext(styled('div'), 'footer');
export const SheetTitle = withContext(styled(SheetPrimitive.Title), 'title');
export const SheetDescription = withContext(styled(SheetPrimitive.Description), 'description');
