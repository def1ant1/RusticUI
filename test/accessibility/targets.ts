/**
 * Manifest of documentation and gallery routes audited by the Playwright + axe
 * harness. Each entry captures the canonical slug plus configuration knobs so
 * the suite scales without ad-hoc conditionals. Keep the list intentionally
 * small and high-signal—each page should map to an end-to-end scenario we rely
 * on in release checklists.
 */
export interface AccessibilityTarget {
  /** Human-friendly identifier used for filenames and Playwright titles. */
  slug: string;
  /** Absolute or relative path navigated from the configured base URL. */
  path: string;
  /**
   * Optional CSS selector that must resolve before the axe analysis executes.
   * Use this when a page hydrates client-side widgets asynchronously.
   */
  readySelector?: string;
  /**
   * Additional wait (in milliseconds) after the selector stabilises. Prefer
   * keeping this `undefined`; only set when a component continuously mutates the
   * DOM (e.g. animated carousels that introduce focus traps).
   */
  postReadyWaitMs?: number;
  /**
   * axe-core rule overrides used to gate fail conditions.
   */
  axe?: {
    /**
     * Explicit rule identifiers that are tolerated temporarily. These surface as
     * warnings in the Markdown attachment but do not fail the build.
     */
    allowedViolations?: string[];
    /**
     * Highest impact level permitted to pass without failing the suite.
     * Set to `none` (default) to fail on any violation. Escalate cautiously and
     * document the rationale in `notes`.
     */
    maxAllowedImpact?: ImpactGate;
  };
  /** Additional context rendered in the Markdown report. */
  notes?: string[];
}

/**
 * Impact levels emitted by axe-core. `none` is an xtask-specific escape hatch
 * that treats every violation as blocking.
 */
export type ImpactGate = 'none' | 'minor' | 'moderate' | 'serious' | 'critical';

export const accessibilityTargets: AccessibilityTarget[] = [
  {
    slug: 'docs-home',
    path: '/',
    readySelector: '#main-content',
    notes: [
      'Validates the navigation shell, hero banner, and search affordances.',
      'Catches regressions caused by marketing campaigns modifying the landing layout.',
    ],
  },
  {
    slug: 'material-buttons',
    path: '/material-ui/react-button/',
    readySelector: '#main-content',
    notes: [
      'Ensures canonical Material button demos retain labelled controls.',
      'Historically flagged regressions where SVG icons dropped accessible titles.',
    ],
  },
  {
    slug: 'joy-overview',
    path: '/joy-ui/getting-started/overview/',
    readySelector: '#main-content',
    notes: [
      'Covers Joy UI overview callouts and quick-start templates.',
      'Exercises tabbed demos that previously failed to forward aria attributes.',
    ],
  },
  {
    slug: 'examples-gallery',
    path: '/examples/quick-start-gallery/',
    readySelector: '#examples-nav',
    postReadyWaitMs: 1000,
    axe: {
      // The gallery lazy-loads iframes with custom sandboxed content. The
      // `frame-title` rule cannot observe the inner frame label until the SSR
      // snapshot hydrates, so we temporarily mark it as non-blocking.
      allowedViolations: ['frame-title'],
      maxAllowedImpact: 'moderate',
    },
    notes: [
      'Audits the shared gallery shell that embeds Dioxus, Leptos, Sycamore, and React examples.',
      'Hydration races can emit transient `frame-title` warnings; these are tracked but do not fail CI while the iframe API is refactored.',
    ],
  },
];
