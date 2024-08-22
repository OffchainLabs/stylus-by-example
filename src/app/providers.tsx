'use client'
import posthog from 'posthog-js'
import { PostHogProvider } from 'posthog-js/react'
import dynamic from "next/dynamic";

if (typeof window !== 'undefined') {
  posthog.init(process.env.NEXT_PUBLIC_POSTHOG_KEY, {
    api_host: process.env.NEXT_PUBLIC_POSTHOG_HOST,
    person_profiles: 'identified_only',
    capture_pageview: false // Disable automatic pageview capture, as we capture manually
  })
}

export function PHProvider ({ children }) {
  return <PostHogProvider client={posthog}>{children}</PostHogProvider>
}

const PostHogPageView = dynamic(() => import('./PostHogPageView'), {
  ssr: false,
})
