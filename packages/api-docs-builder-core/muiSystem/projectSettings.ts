import path from 'path';
import { LANGUAGES } from 'docs/config';
import { ProjectSettings } from '@mui-internal/api-docs-builder';
import findApiPages from '@mui-internal/api-docs-builder/utils/findApiPages';
import generateUtilityClass, { isGlobalState } from '@mui/utils/generateUtilityClass';
import { resolvePackageSourceRoot, rustDocFlags } from '../../../scripts/rustDocAutomation.js';
import { getSystemComponentInfo } from './getSystemComponentInfo';

const systemRoot = resolvePackageSourceRoot('mui-system');
// Avoid duplicating work once the Rust metadata is considered the authority.
const typeScriptProjects: ProjectSettings['typeScriptProjects'] = rustDocFlags.shouldSkipArchives
  ? []
  : [
      {
        name: 'system',
        rootPath: systemRoot,
        entryPointPath: 'src/index.d.ts',
      },
    ];

export const projectSettings: ProjectSettings = {
  output: {
    apiManifestPath: path.join(process.cwd(), 'docs/data/system/pagesApi.js'),
  },
  // System APIs now sync from Rust. When the automation asserts authority we
  // let the JSON pipeline drive everything and avoid touching the archive.
  typeScriptProjects,
  getApiPages: () => findApiPages('docs/pages/system/api'),
  getComponentInfo: getSystemComponentInfo,
  translationLanguages: LANGUAGES,
  skipComponent(filename) {
    return (
      filename.match(
        /(ThemeProvider|CssVarsProvider|DefaultPropsProvider|GlobalStyles|InitColorSchemeScript)/,
      ) !== null
    );
  },
  translationPagesDirectory: 'docs/translations/api-docs',
  generateClassName: generateUtilityClass,
  isGlobalClassName: isGlobalState,
};
