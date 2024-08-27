import { css, cx } from '@/styled-system/css';
import Link from 'next/link';

interface PageCardProps {
  title: string;
  description: string;
  route: string;
}

export function PageCard({ title, description, route }: PageCardProps) {
  return (
    <Link
      href={route}
      className={cx(
        'group',
        css({
          display: 'block',
          borderWidth: '1px',
          borderColor: 'stone.300',
          borderRadius: 'lg',
          p: '6',
        }),
      )}
    >
      <h2
        className={css({
          fontSize: 'xl',
          fontWeight: 'bold',
          fontFamily: 'heading',
          mb: '2',
          _groupHover: {
            color: 'pink.400',
          },
        })}
      >
        {title}
      </h2>
      <p>{description}</p>
    </Link>
  );
}
