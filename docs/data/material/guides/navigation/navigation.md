# Navigation orchestration

<p class="description">Build enterprise-grade navigation with Tabs and Drawer components that scale across routers, teams, and release pipelines.</p>

Material&nbsp;UI ships mature navigation primitives, but enterprise success depends on wiring them into routing, state management, theming, and automation layers consistently.
This guide captures the repeatable patterns that keep Tabs and Drawer experiences resilient in mission-critical deployments.

:::success
When you standardize on the patterns below, your teams can scaffold new navigation flows through shared generators, instead of recoding Tabs or Drawer behaviors by hand every sprint.
:::

## Routing integration playbooks

Tabs and Drawer only feel native when they stay in lockstep with the active route. Adopt one of the following patterns depending on your routing stack and hosting model.

### Declarative routers (React Router, TanStack Router)

1. Model navigation items as data, including pathname, label, and feature flags.
2. Feed that data into both your router and your Tabs/Drawer components so the render tree remains single-sourced.
3. Synchronize the active tab or drawer section via the router's location object.

```tsx
// navigation-registry.ts centralizes routes for routers, tabs, and drawers.
export interface NavigationItem {
  readonly id: string;
  readonly label: string;
  readonly path: string;
  readonly icon?: React.ReactNode;
  readonly featureFlag?: string;
}

export const NAVIGATION_ITEMS: readonly NavigationItem[] = [
  { id: 'overview', label: 'Overview', path: '/workspace/overview' },
  { id: 'analytics', label: 'Analytics', path: '/workspace/analytics', featureFlag: 'beta-insights' },
  { id: 'settings', label: 'Settings', path: '/workspace/settings' },
];
```

```tsx
// Tabs + router coupling using the centralized registry.
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import { useLocation, useNavigate } from 'react-router';
import { NAVIGATION_ITEMS } from './navigation-registry';

export function WorkspaceTabs() {
  const location = useLocation();
  const navigate = useNavigate();
  const activeItem = NAVIGATION_ITEMS.find((item) => location.pathname.startsWith(item.path));

  return (
    <Tabs
      value={activeItem?.id ?? false}
      onChange={(_, id) => {
        const destination = NAVIGATION_ITEMS.find((item) => item.id === id);
        if (destination) navigate(destination.path);
      }}
      aria-label="Workspace sections"
    >
      {NAVIGATION_ITEMS.map((item) => (
        <Tab
          key={item.id}
          value={item.id}
          label={item.label}
          icon={item.icon}
          iconPosition="start"
          disabled={item.featureFlag === 'beta-insights'}
        />
      ))}
    </Tabs>
  );
}
```

### Next.js App Router

- Co-locate the navigation registry in `/app/(config)/navigation.ts` and reuse it from server components so that `<Drawer />` menus remain in sync with static generation and `generateStaticParams`.
- Wrap Tabs in [`useSelectedLayoutSegment`](https://nextjs.org/docs/app/api-reference/functions/use-selected-layout-segment) to highlight the active segment without relying on client-side effects.
- Export a reusable `NavigationContextProvider` that exposes helpers like `openDrawer`, `closeDrawer`, and `setActiveTab` so client components can hydrate with the same server-authored defaults.

### Micro-frontends and modular federated shells

- Host the navigation registry in a shared module (for example, a pnpm workspace package) and version it alongside contracts consumed by each federated runtime.
- Expose imperative connectors—`registerNavigationSlot`, `registerDrawerRoute`—that micro-frontends call during bootstrap so the shell can merge contributions into a single Tabs/Drawer presentation.
- Emit analytics and audit events from the shell, not individual micro-frontends, to maintain consistent security posture.

## Controlled, manual, and hybrid state patterns

Tabs and Drawer render predictably only when their state ownership is explicit:

- **Controlled Tabs:** Supply the `value` and `onChange` props to sync with Redux, Zustand, or React Context stores. Prefer this for cross-tab persistence and analytics logging.
- **Manual Drawer orchestration:** Keep Drawer `open` state alongside layout breakpoints. Use `useMediaQuery` to collapse permanent drawers into temporary overlays when the viewport shrinks.
- **Hybrid fallback:** Initialize Drawer open state from server-rendered layout metadata, then allow user overrides stored in `localStorage` or your preference service.

```tsx
function PlatformDrawer() {
  const prefersRail = useMediaQuery((theme) => theme.breakpoints.up('lg'));
  const [open, setOpen] = React.useState(prefersRail);

  React.useEffect(() => {
    setOpen(prefersRail);
  }, [prefersRail]);

  return (
    <Drawer
      variant={prefersRail ? 'permanent' : 'temporary'}
      open={open}
      onClose={() => setOpen(false)}
      ModalProps={{ keepMounted: true }}
    >
      {/* Centralized navigation list renders here */}
    </Drawer>
  );
}
```

## Theming hooks and design system alignment

- Tap into [`useTheme`](/material-ui/customization/how-to-customize/#using-the-theme-in-a-component) to consume tokens for drawer widths, tab indicators, and surface colors instead of hard-coded values.
- Extend `createTheme` with custom `components.MuiTabs.styleOverrides` and `components.MuiDrawer.defaultProps` so downstream teams inherit a consistent navigation aesthetic by default.
- Expose utility hooks such as `useNavigationSurface()` in your design system package to centralize spacing, icon sizing, and typography decisions.

:::info
Pair Tabs and Drawer with [`ThemeProvider`](/material-ui/customization/theming/) instances that read customer-specific palettes or role-based density overrides. This enables white-label programs without branching your navigation components.
:::

## Enterprise-scale deployment strategies

| Challenge | Recommendation |
| --- | --- |
| Code-splitting navigation payloads | Lazy load rarely accessed drawer groups via `<Suspense>` boundaries while keeping top-level Tabs eagerly rendered. |
| Progressive enhancement | Render Drawer navigation items server-side so search crawlers and monitoring tools detect full IA even without JavaScript. |
| Observability | Instrument Tabs `onChange` and Drawer `onClose` handlers to emit analytics and audit logs through a shared telemetry SDK. |
| Feature rollouts | Wrap navigation items in experimentation guards (LaunchDarkly, Optimizely) and ship pre-approved fallbacks to keep CI/CD pipelines green. |

When scaling beyond a single repo, define a navigation package that exports React components, analytics contracts, and test helpers. Publish it through your internal registry so product teams can import the same primitives with a single dependency.

## Accessibility conformance

- Respect [WAI-ARIA Tabs guidance](https://www.w3.org/WAI/ARIA/apg/patterns/tabs/) by ensuring every `<Tab />` is associated with a `<TabPanel />` via `aria-controls` and `id`.
- Assign Drawer `aria-label` or `aria-labelledby` values that describe the navigation purpose, and ensure temporary drawers trap focus until dismissed.
- Validate keyboard coverage through automated suites ([`@testing-library/user-event`](https://testing-library.com/docs/ecosystem-user-event/)) and nightly manual audits using assistive technology like NVDA and VoiceOver.
- Embed [`aria-live`](/material-ui/react-snackbar/#accessibility) regions to announce Drawer state transitions for screen reader users.

## Responsive design blueprint

- Combine [`useMediaQuery`](/material-ui/react-use-media-query/) with [`Hidden` variants](/material-ui/react-hidden/) or [CSS breakpoints](/material-ui/customization/breakpoints/) to transition Drawer variants (permanent → temporary) at appropriate viewport widths.
- Augment Tabs with scroll buttons, dynamic label truncation, or overflow menus so they remain usable on compact screens.
- Capture viewport preferences (for example, drawer collapse state) in a profile service so that experiences stay consistent across devices.

## Automation and CI/CD guardrails

- Gate every navigation change with CI jobs that run [`pnpm docs:link-check`](https://mui.com/material-ui/guides/testing/#continuous-integration) plus custom route validation to catch broken deep links.
- Share ESLint configurations enforcing `aria-*` attributes and import paths (for example, no relative imports bypassing the navigation package).
- Schedule visual regression suites (Argos, Chromatic) to confirm Drawer overlays and tab indicators render across browsers after each release.
- Generate Storybook stories for core navigation states and run them through accessibility scanners (axe, pa11y) in CI to prevent regressions before production.

## Further reading

- [Tabs component documentation](/material-ui/react-tabs/)
- [Drawer component documentation](/material-ui/react-drawer/)
- [Routing integrations](/material-ui/integrations/routing/)
- [Responsive UI guide](/material-ui/guides/responsive-ui/)
