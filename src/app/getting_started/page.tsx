import { grid } from '@/styled-system/patterns';
import { PageCard } from '@/components/app/page_card';
import { gettingStarted } from '@/data/routes';

export default function PageRoot() {
  return (
    <div className={grid({ columns: 2, gap: '6' })}>
      {gettingStarted.map((routeInfo) => (
        <PageCard {...routeInfo} />
      ))}
    </div>
  );
}
