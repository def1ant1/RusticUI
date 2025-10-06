import * as React from 'react';
import Alert from '@mui/material/Alert';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import NoSsr from '@mui/material/NoSsr';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';
import LaunchRoundedIcon from '@mui/icons-material/LaunchRounded';
import {
  SandpackCodeEditor,
  SandpackLayout,
  SandpackPreview,
  SandpackProvider,
} from '@codesandbox/sandpack-react';
import {
  buildQuickStartButtonSandbox,
  QUICK_START_BUTTON_ENTRY_FILE,
  QUICK_START_BUTTON_PREVIEW_FILE,
} from './QuickStartButtonGenerator';

/**
 * Inline notes clarify how the docs consume the shared generator:
 *  - We memoize the sandbox descriptor so renders never allocate new file maps.
 *  - Sandpack runs exclusively on the client (wrapped in NoSsr) to avoid SSR mismatches.
 *  - The StackBlitz launcher calls into the generator output which keeps browser tooling aligned
 *    with CI scripts that import the same helper.
 */
export default function QuickStartButtonGallery() {
  const sandbox = React.useMemo(() => buildQuickStartButtonSandbox(), []);

  const handleOpenStackBlitz = React.useCallback(() => {
    sandbox.openStackBlitz();
  }, [sandbox]);

  return (
    <Stack spacing={2} sx={{ my: 3 }}>
      <Typography component="h2" variant="h5" fontWeight={700}>
        Quick-start button playground
      </Typography>
      <Alert severity="info">
        This embedded Sandpack runs the same generator that powers the StackBlitz export and CLI
        snapshot. Editing the files below mirrors the structure checked into the repository, making
        it obvious when a change requires a sandbox refresh.
      </Alert>
      <NoSsr defer>
        <SandpackProvider
          template="react-ts"
          files={sandbox.files}
          customSetup={{
            dependencies: sandbox.dependencies,
            entry: QUICK_START_BUTTON_ENTRY_FILE.replace('/', ''),
          }}
          options={{
            visibleFiles: sandbox.visibleFiles as string[],
            activeFile: QUICK_START_BUTTON_PREVIEW_FILE,
            recompileMode: 'lazy',
            recompileDelay: 400,
          }}
        >
          <SandpackLayout>
            <SandpackPreview
              showOpenInCodeSandbox={false}
              showRefreshButton
              style={{ minHeight: 420 }}
            />
            <SandpackCodeEditor
              showLineNumbers
              showTabs
              wrapContent
              style={{ minHeight: 420 }}
            />
          </SandpackLayout>
        </SandpackProvider>
      </NoSsr>
      <Box>
        <Stack
          direction={{ xs: 'column', sm: 'row' }}
          spacing={2}
          justifyContent="space-between"
          alignItems={{ xs: 'stretch', sm: 'center' }}
        >
          <Typography variant="body2" color="text.secondary">
            Run <code>pnpm --dir docs sandbox:quick-start -- --check</code> in CI to confirm the
            exported JSON snapshot still matches the generator output, then use the button to inspect
            the same project in StackBlitz if a review requires deeper edits.
          </Typography>
          <Button
            variant="contained"
            color="primary"
            startIcon={<LaunchRoundedIcon />}
            onClick={handleOpenStackBlitz}
            sx={{ alignSelf: { xs: 'stretch', sm: 'flex-start' } }}
          >
            Open in StackBlitz
          </Button>
        </Stack>
      </Box>
    </Stack>
  );
}
