'use client';

import { css, cx } from '@/styled-system/css';
import { CopyIcon, CheckIcon } from '@radix-ui/react-icons';
import { Button } from '@/components/ui/button';

interface CopyButtonProps {
  content?: string;
}

export async function onCopy(content: string) {
  try {
    await navigator.clipboard.writeText(content);
  } catch (err) {
    console.error('Failed to copy: ', err);
  }
}

export function CopyButton({ content }: CopyButtonProps) {
  return (
    <>
      <Button
        variant="ghost"
        className={cx(
          'group',
          css({
            background: 'gray.500',
            opacity: '0.3',
            color: 'white',
            h: '36px',
            w: '36px',
            px: '0',
            position: 'absolute',
            top: '8px',
            right: '8px',
            rounded: 'sm',
            _hover: {
              cursor: 'pointer',
              opacity: '0.7',
            },
            _active: {
              opacity: '0.7',
              background: 'lime.400',
            },
            _focus: {
              opacity: '0.7',
              background: 'lime.400',
            },
          }),
        )}
        onClick={(e) => {
          if (content) {
            onCopy(content);
          }

          const target = e.currentTarget;

          setTimeout(() => {
            target.blur();
          }, 1000);
        }}
      >
        <CopyIcon
          className={css({
            h: '24px',
            w: '24px',
            _groupActive: {
              display: 'none',
            },
            _groupFocus: {
              display: 'none',
            },
          })}
        />
        <CheckIcon
          className={css({
            h: '24px',
            w: '24px',
            display: 'none',
            _groupActive: {
              display: 'block',
            },
            _groupFocus: {
              display: 'block',
            },
          })}
        />
      </Button>
    </>
  );
}
