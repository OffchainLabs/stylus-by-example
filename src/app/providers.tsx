'use client';
import posthog from 'posthog-js';
import { PostHogProvider } from 'posthog-js/react';

import { initializePostHog } from '@/lib/analytics';

// Initialized at module scope rather than in an effect: child effects run before
// parent effects in React, so a provider that initialized in useEffect would let
// PostHogPageView's first $pageview fire before posthog.init.
initializePostHog();

export function PHProvider({ children }: { children: React.ReactNode }) {
  return <PostHogProvider client={posthog}>{children}</PostHogProvider>;
}
