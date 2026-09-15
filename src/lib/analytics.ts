/**
 * PostHog analytics configuration.
 */

import posthog from 'posthog-js';
import type { BeforeSendFn, PostHogConfig } from 'posthog-js';

/**
 * Device, locale and timezone properties PostHog collects by default. They are
 * dropped client-side before any event leaves the browser.
 */
export const POSTHOG_DEVICE_PROPERTY_DENYLIST: readonly string[] = [
  // Hardware and screen
  '$device',
  '$device_type',
  '$device_model',
  '$screen_height',
  '$screen_width',
  '$viewport_height',
  '$viewport_width',
  // OS and browser fingerprint
  '$os',
  '$os_name',
  '$os_version',
  '$browser',
  '$browser_version',
  '$browser_type',
  // Locale and timezone
  '$browser_language',
  '$browser_language_prefix',
  '$timezone',
  '$timezone_offset',
];

/**
 * Scroll-depth values PostHog attaches to $pageleave and the following $pageview.
 * Their raw pixel values embed the viewport height (scrollY + clientHeight).
 * `disable_scroll_properties` stops them at source; this is the backstop.
 */
export const POSTHOG_SCROLL_PROPERTY_DENYLIST: readonly string[] = [
  '$prev_pageview_last_scroll',
  '$prev_pageview_last_scroll_percentage',
  '$prev_pageview_max_scroll',
  '$prev_pageview_max_scroll_percentage',
  '$prev_pageview_last_content',
  '$prev_pageview_last_content_percentage',
  '$prev_pageview_max_content',
  '$prev_pageview_max_content_percentage',
];

/**
 * Catches device properties a future posthog-js may add under a family we already
 * deny above, so the denylist does not silently go stale on SDK upgrades.
 * Deliberately excludes `raw_user_agent` (required for the cookieless hash) and
 * `geoip` (`$geoip_disable` is a flag we send).
 */
export const POSTHOG_DEVICE_PROPERTY_PATTERN = /^\$(?:screen|viewport|device|os|browser|timezone)/;

/**
 * Runs last, after property_denylist and after $set/$set_once are built.
 * Must never throw: posthog-js drops the event entirely if it does.
 */
export const sanitizePostHogEvent: BeforeSendFn = (event) => {
  if (!event?.properties) {
    return event;
  }

  for (const key of Object.keys(event.properties)) {
    if (POSTHOG_DEVICE_PROPERTY_PATTERN.test(key)) {
      delete event.properties[key];
    }
  }

  return event;
};

/**
 * Shared PostHog configuration.
 */
export const POSTHOG_CONFIG: Partial<PostHogConfig> = {
  api_host: process.env.NEXT_PUBLIC_POSTHOG_HOST || 'https://us.i.posthog.com',

  // Cookieless: no cookies, no localStorage, no sessionStorage.
  cookieless_mode: 'always',
  persistence: 'memory',
  disable_persistence: true,
  person_profiles: 'never',

  // Device information is never captured; the user agent stays for the cookieless hash.
  property_denylist: [...POSTHOG_DEVICE_PROPERTY_DENYLIST, ...POSTHOG_SCROLL_PROPERTY_DENYLIST],
  before_send: sanitizePostHogEvent,
  // Do not request User-Agent Client Hints, which the SDK uses to derive $device_model.
  disableDeviceModel: true,
  // Mask ad-network click IDs (gclid, fbclid, ...) inside captured URLs.
  mask_personal_data_properties: true,

  // Manual, explicit tracking only.
  autocapture: false,
  capture_pageview: false,
  capture_pageleave: true,
  // Never measure scroll position; the values would leak the viewport height.
  disable_scroll_properties: true,

  // Features that persist state or ship extra device data are off.
  disable_session_recording: true,
  disable_surveys: true,
  disable_web_experiments: true,
  disable_external_dependency_loading: true,
  capture_performance: false,
  capture_heatmaps: false,
  capture_dead_clicks: false,
  advanced_disable_flags: true,
  advanced_disable_toolbar_metrics: true,

  loaded: (instance) => {
    // Ask PostHog not to derive location from the request IP.
    instance.register({ $geoip_disable: true });
    // Dev-only console logging. Note: debug mode writes a `ph_debug` key to
    // localStorage, which is why it is limited to local development.
    if (process.env.NODE_ENV === 'development') {
      instance.debug();
    }
  },
};

/**
 * Ad-network click IDs. Each one identifies a single ad click, so they are
 * personal data under GDPR. PostHog masks them out of the `$current_url` it
 * derives itself, but not out of a URL a caller passes in explicitly, which is
 * what PostHogPageView has to do on client-side navigation. Stripped here.
 * `utm_*` is deliberately left in: it names a campaign, not a person.
 */
export const URL_TRACKING_PARAMS: readonly string[] = [
  '_kx',
  'dclid',
  'epik',
  'fbclid',
  'gad_source',
  'gbraid',
  'gclid',
  'gclsrc',
  'igshid',
  'irclid',
  'li_fat_id',
  'mc_cid',
  'msclkid',
  'qclid',
  'rdt_cid',
  'sccid',
  'ttclid',
  'twclid',
  'wbraid',
];

/**
 * Remove click IDs from a URL before it is attached to an event. Falls back to
 * the origin and path if the URL cannot be parsed, so a malformed URL can never
 * leak its query string.
 */
export const sanitizeUrl = (url: string): string => {
  try {
    const parsed = new URL(url);
    for (const param of URL_TRACKING_PARAMS) {
      parsed.searchParams.delete(param);
    }
    return parsed.toString();
  } catch {
    return url.split('?')[0];
  }
};

/**
 * localStorage key posthog-js used before cookieless mode, when `persistence`
 * defaulted to "localStorage+cookie" with the default `persistence_name`.
 */
export const legacyStorageKey = (token: string): string => `ph_${token}_posthog`;

/**
 * Remove the identifier left behind by the previous configuration. Cookieless
 * mode never reads it, but leaving it in place keeps an old distinct_id and
 * device_id on the visitor's device.
 */
export const removeLegacyStorage = (token: string): void => {
  try {
    window.localStorage.removeItem(legacyStorageKey(token));
  } catch {
    // Storage can be unavailable or blocked; there is nothing to clean up then.
  }
};

/**
 * Initialize the PostHog client. Called once, from the provider.
 */
export const initializePostHog = (): void => {
  const token = process.env.NEXT_PUBLIC_POSTHOG_KEY;

  if (typeof window === 'undefined' || !token) {
    return;
  }

  removeLegacyStorage(token);
  posthog.init(token, POSTHOG_CONFIG);
};
