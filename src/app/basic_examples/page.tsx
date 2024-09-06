import { grid } from '@/styled-system/patterns';
import { PageCard } from '@/components/app/page_card';
import { basicExamples } from '@/data/routes';

export default function PageRoot() {
  return (
    <div className={grid({ columns: 2, gap: '6' })}>
      {basicExamples.map((routeInfo) => (
        <PageCard {...routeInfo} />
      ))}
    </div>
  );
}
