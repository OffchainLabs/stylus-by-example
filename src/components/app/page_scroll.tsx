import { css } from '@/styled-system/css';

export function PageScroll({ children }: { children: React.ReactNode }) {
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
          // background: theme === "dark" ? "stone.900" : "stone.200",
        },
      })}
    >
      {children}
    </div>
  );
}
