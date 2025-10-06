import type { SandpackFiles } from '@codesandbox/sandpack-react';
import stackBlitz from 'docs/src/modules/sandbox/StackBlitz';
import docsPackage from '../../../package.json' assert { type: 'json' };

/**
 * Structured description of the Sandpack project that powers the quick-start button gallery.
 * Keeping this shape centralized allows CI automation, docs pages, and ad-hoc scripts to reuse
 * the exact same source of truth.
 */
export interface QuickStartButtonSandbox {
  /** Entry file executed by Sandpack when bootstrapping the virtual workspace. */
  readonly entryFile: string;
  /** File opened by default in the editor so engineers immediately see the orchestrating surface. */
  readonly previewFile: string;
  /** Complete set of files provided to Sandpack (includes metadata required by the renderer). */
  readonly files: SandpackFiles;
  /** Raw string versions of each file for scripts that need to emit JSON snapshots. */
  readonly fileEntries: Record<string, string>;
  /** Helpful ordering for the tab strip so docs stay in sync with local editors. */
  readonly visibleFiles: readonly string[];
  /** Dependency versions pinned to the docs workspace to avoid manual version drift. */
  readonly dependencies: Record<string, string>;
  /** Convenience launcher that opens the exact same project in StackBlitz for deeper editing. */
  readonly openStackBlitz: () => void;
  /** Human readable name reused across documentation copy and automation logs. */
  readonly title: string;
}

/**
 * Lightweight helper that resolves dependency versions from the docs workspace manifest.
 * We favour the workspace-pinned versions over "latest" to avoid the playground diverging from
 * the production bundle served by the documentation site.
 */
function resolveWorkspaceVersion(name: string, fallback: string = 'latest'): string {
  const pkg = docsPackage as { dependencies?: Record<string, string>; devDependencies?: Record<string, string> };
  return pkg.dependencies?.[name] ?? pkg.devDependencies?.[name] ?? fallback;
}

/** Files that Sandpack should treat as the entry point and the default editor focus. */
export const QUICK_START_BUTTON_ENTRY_FILE = '/index.tsx';
export const QUICK_START_BUTTON_PREVIEW_FILE = '/App.tsx';

/**
 * Source for the shared button. Inline documentation is intentionally verbose so engineers skimming
 * the live playground immediately understand the analytics hooks and automation attributes.
 */
const quickStartButtonSource = String.raw`/**
 * The shared quick-start button keeps automation identifiers, analytics hooks, and styling in one
 * place so React, Rust, and docs surfaces all hydrate the same markup. The Sandpack gallery loads
 * this module directly which means any edits flow to StackBlitz, Storybook, and downstream examples
 * without manual copy/paste.
 */
import * as React from 'react';
import Button from '@mui/material/Button';
import Tooltip from '@mui/material/Tooltip';
import LaunchIcon from '@mui/icons-material/RocketLaunchRounded';

export interface QuickStartButtonProps {
  /** Destination URL for the primary quick-start action. */
  readonly href: string;
  /** Deterministic automation identifier that QA suites replay across frameworks. */
  readonly automationId: string;
  /** Visible label rendered inside the Material button. */
  readonly label: string;
  /** Optional analytics identifier to wire structured telemetry. */
  readonly analyticsId?: string;
  /** Allow alternate visual treatments without touching call sites. */
  readonly variant?: 'contained' | 'outlined';
}

export function QuickStartButton({
  href,
  automationId,
  label,
  analyticsId = 'docs.quick-start.button',
  variant = 'contained',
}: QuickStartButtonProps) {
  return (
    <Tooltip
      arrow
      placement="top"
      title="Launch the shared Material quick-start workspace"
    >
      <Button
        data-rustic-app-action={automationId}
        data-rustic-analytics={analyticsId}
        href={href}
        variant={variant}
        size="large"
        color="primary"
        disableElevation
        sx={{
          px: 4,
          py: 1.5,
          fontWeight: 600,
          textTransform: 'none',
          borderRadius: 999,
        }}
        endIcon={<LaunchIcon fontSize="small" />}
      >
        {label}
      </Button>
    </Tooltip>
  );
}

export default QuickStartButton;
`;

/**
 * Orchestrating application that wraps the shared CTA with the standard Material theme and copy.
 * The comments mirror the inline reminders in our Rust examples so teams following the docs see the
 * same rationale in every environment.
 */
const appSource = String.raw`import * as React from 'react';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';
import { ThemeProvider } from '@mui/material/styles';
import QuickStartButton from './QuickStartButton';
import theme from './theme';

const CTA_URL = 'https://github.com/RusticUI/rusticui/tree/main/examples';

export default function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box
        sx={{
          minHeight: '100vh',
          bgcolor: 'background.default',
          color: 'text.primary',
          display: 'grid',
          placeItems: 'center',
          p: 4,
        }}
      >
        <Stack spacing={3} alignItems="center" sx={{ maxWidth: 520, textAlign: 'center' }}>
          <Typography component="h1" variant="h4" fontWeight={700}>
            RusticUI quick-start CTA
          </Typography>
          <Typography variant="body1">
            Ship the same Material hero button across React, Yew, Leptos, and Sycamore adapters without
            duplicating analytics or automation glue. The shared generator feeds this preview and the
            StackBlitz export so CI only needs to validate one source file.
          </Typography>
          <QuickStartButton
            href={CTA_URL}
            automationId="app-quick-start-primary"
            label="Bootstrap the shared Material shell"
          />
        </Stack>
      </Box>
    </ThemeProvider>
  );
}
`;

/**
 * Theme file that mirrors the Material palette and typography tweaks used across the Rust demos.
 * Keeping these values alongside the Sandpack ensures visual parity with the SSR snapshots.
 */
const themeSource = String.raw`import { createTheme } from '@mui/material/styles';

const theme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#0066CC',
    },
    secondary: {
      main: '#5C4B8A',
    },
  },
  typography: {
    fontFamily: 'Inter, "Roboto", "Helvetica", "Arial", sans-serif',
    button: {
      textTransform: 'none',
      fontWeight: 600,
      letterSpacing: 0.2,
    },
  },
});

export default theme;
`;

/**
 * Minimal React 18 entry point. The guard around the root element makes the bundle fail-fast during
 * CI runs so automation never records a false positive when the template changes.
 */
const indexSource = String.raw`import * as React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';

const container = document.getElementById('root');

if (!container) {
  throw new Error('Missing #root element. The docs generator keeps this in sync with Sandpack.');
}

createRoot(container).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
`;

/** Ordered list of files used by Sandpack and StackBlitz. */
const rawFileEntries: Record<string, string> = {
  'App.tsx': appSource,
  'QuickStartButton.tsx': quickStartButtonSource,
  'theme.ts': themeSource,
  'index.tsx': indexSource,
};

/**
 * Builds the sandbox configuration consumed by the docs playground, StackBlitz launcher, and CLI
 * export script.
 */
export function buildQuickStartButtonSandbox(): QuickStartButtonSandbox {
  const title = 'RusticUI quick-start Material button';

  const files: SandpackFiles = Object.fromEntries(
    Object.entries(rawFileEntries).map(([filePath, code]) => [
      `/${filePath}`,
      {
        code,
        active: filePath === QUICK_START_BUTTON_PREVIEW_FILE.replace('/', ''),
      },
    ]),
  );

  const visibleFiles = Object.keys(files);

  const dependencies: Record<string, string> = {
    '@emotion/react': resolveWorkspaceVersion('@emotion/react'),
    '@emotion/styled': resolveWorkspaceVersion('@emotion/styled'),
    '@mui/icons-material': resolveWorkspaceVersion('@mui/icons-material'),
    '@mui/material': resolveWorkspaceVersion('@mui/material'),
    react: resolveWorkspaceVersion('react', '^19.1.1'),
    'react-dom': resolveWorkspaceVersion('react-dom', '^19.1.1'),
  };

  const stackBlitzTemplate = stackBlitz.createMaterialTemplate({
    title,
    githubLocation: '/docs/src/components/examples/QuickStartButtonGallery.tsx',
    codeVariant: 'TS',
    files: rawFileEntries,
  });

  return {
    title,
    entryFile: QUICK_START_BUTTON_ENTRY_FILE,
    previewFile: QUICK_START_BUTTON_PREVIEW_FILE,
    files,
    fileEntries: rawFileEntries,
    visibleFiles,
    dependencies,
    openStackBlitz: () => stackBlitzTemplate.openStackBlitz('src/App'),
  };
}
