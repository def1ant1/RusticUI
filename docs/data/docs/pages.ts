import type { MuiPage } from 'docs/src/MuiPage';
import standardNavIcons from 'docs/src/modules/components/AppNavIcons';

const pages: readonly MuiPage[] = [
  {
    pathname: '/getting-started-group',
    title: 'Getting started',
    /**
     * The quick-start doc seeds every other workspace bootstrap flow (Rust, React,
     * StackBlitz, etc.). Surfacing it at the top of the drawer keeps new visitors
     * on the happy path before they dive into framework specific adapters.
     */
    children: [{ pathname: '/getting-started/quick-start', title: 'Quick start' }],
  },
  {
    pathname: '/examples',
    title: 'Examples',
    /**
     * Mirror the example landing page so the drawer exposes both automation-heavy
     * blueprints and the live quick-start gallery. Centralising these entries keeps
     * the drawer aligned with the generator + sandbox tooling we ship for CI.
     */
    children: [
      { pathname: '/examples', title: 'Overview' },
      { pathname: '/examples/quick-start-gallery', title: 'Quick-start gallery' },
      { pathname: '/examples/automation', title: 'Automation blueprints' },
      {
        pathname: '/examples/selection-controls-telemetry',
        title: 'Selection control telemetry walkthrough',
      },
    ],
  },
  { pathname: 'https://mui.com/versions/' },
  {
    pathname: 'https://mui.com/store/',
    title: 'Templates',
    icon: standardNavIcons.ReaderIcon,
    linkProps: {
      'data-ga-event-category': 'store',
      'data-ga-event-action': 'click',
      'data-ga-event-label': 'sidenav',
    },
  },
  { pathname: 'https://mui.com/blog/', title: 'Blog', icon: standardNavIcons.BookIcon },
];

export default pages;
