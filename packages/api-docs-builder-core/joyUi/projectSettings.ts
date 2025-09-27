import path from 'path';
import { LANGUAGES } from 'docs/config';
import { ProjectSettings } from '@mui-internal/api-docs-builder';
import findApiPages from '@mui-internal/api-docs-builder/utils/findApiPages';
import generateUtilityClass, { isGlobalState } from '@mui/utils/generateUtilityClass';
import { resolvePackageSourceRoot, rustDocFlags } from '../../../scripts/rustDocAutomation.js';
import { getJoyUiComponentInfo } from './getJoyUiComponentInfo';

const joyRoot = resolvePackageSourceRoot('mui-joy');
// Allow CI to point the docs builder at Rust-emitted JSON when the archived
// Joy UI bundle is no longer the source of truth.
const typeScriptProjects: ProjectSettings['typeScriptProjects'] = rustDocFlags.shouldSkipArchives
  ? []
  : [
      {
        name: 'joy',
        rootPath: joyRoot,
        entryPointPath: 'src/index.ts',
      },
    ];

export const projectSettings: ProjectSettings = {
  output: {
    apiManifestPath: path.join(process.cwd(), 'docs/data/joy/pagesApi.js'),
  },
  // The automation mirrors Joy UI API data from Rust when the flags demand it,
  // otherwise we hydrate from the archived React sources.
  typeScriptProjects,
  getApiPages: () => findApiPages('docs/pages/joy-ui/api'),
  getComponentInfo: getJoyUiComponentInfo,
  translationLanguages: LANGUAGES,
  skipComponent(filename: string) {
    // Container's demo isn't ready
    // GlobalStyles's demo isn't ready
    return (
      filename.match(
        /(ThemeProvider|CssVarsProvider|Container|ColorInversion|GlobalStyles|InitColorSchemeScript)/,
      ) !== null
    );
  },
  translationPagesDirectory: 'docs/translations/api-docs-joy',
  generateClassName: generateUtilityClass,
  isGlobalClassName: isGlobalState,
};
