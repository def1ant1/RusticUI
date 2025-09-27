import path from 'path';
import { LANGUAGES } from 'docs/config';
import { ProjectSettings } from '@mui-internal/api-docs-builder';
import findApiPages from '@mui-internal/api-docs-builder/utils/findApiPages';
import generateUtilityClass, { isGlobalState } from '@mui/utils/generateUtilityClass';
import { resolvePackageSourceRoot, rustDocFlags } from '../../../scripts/rustDocAutomation.js';
import { getMaterialUiComponentInfo } from './getMaterialUiComponentInfo';

const generateClassName = (componentName: string, slot: string, globalStatePrefix = 'Mui') => {
  if (componentName === 'MuiSwipeableDrawer') {
    // SwipeableDrawer uses Drawer classes without modifying them
    return generateUtilityClass('MuiDrawer', slot, globalStatePrefix);
  }

  return generateUtilityClass(componentName, slot, globalStatePrefix);
};

const materialRoot = resolvePackageSourceRoot('mui-material');
const labRoot = resolvePackageSourceRoot('mui-lab');

// When the Rust toolchain is the source of truth we skip loading the archived
// TypeScript packages altogether – automation reads the JSON emitted by the
// crates and mirrors it into the docs.
const typeScriptProjects: ProjectSettings['typeScriptProjects'] = rustDocFlags.shouldSkipArchives
  ? []
  : [
      {
        name: 'material',
        rootPath: materialRoot,
        entryPointPath: [
          'src/index.d.ts',
          'src/PigmentStack/PigmentStack.tsx',
          'src/PigmentContainer/PigmentContainer.tsx',
          'src/PigmentGrid/PigmentGrid.tsx',
        ],
      },
      {
        name: 'lab',
        rootPath: labRoot,
        entryPointPath: 'src/index.d.ts',
      },
    ];

export const projectSettings: ProjectSettings = {
  output: {
    apiManifestPath: path.join(process.cwd(), 'docs/data/material/pagesApi.js'),
  },
  typeScriptProjects,
  getApiPages: () => findApiPages('docs/pages/material-ui/api'),
  getComponentInfo: getMaterialUiComponentInfo,
  translationLanguages: LANGUAGES,
  skipComponent(filename: string) {
    return filename.match(/(ThemeProvider|CssVarsProvider|DefaultPropsProvider)/) !== null;
  },
  translationPagesDirectory: 'docs/translations/api-docs',
  generateClassName,
  isGlobalClassName: isGlobalState,
  // #host-reference
  baseApiUrl: 'https://mui.com',
  nonComponentFolders: [
    'material/getting-started',
    'material/customization',
    'material/experimental-api',
    'material/guides',
    'material/integrations',
    'material/migration',
  ],
};
