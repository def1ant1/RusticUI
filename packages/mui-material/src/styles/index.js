export { default as THEME_ID } from './identifier';
export { default as adaptV4Theme } from './adaptV4Theme';
export {
  hexToRgb,
  rgbToHex,
  hslToRgb,
  decomposeColor,
  recomposeColor,
  getContrastRatio,
  getLuminance,
  emphasize,
  alpha,
  darken,
  lighten,
  css,
  keyframes,
} from '@mui/system';
export { unstable_createBreakpoints } from '@mui/system/createBreakpoints';
// TODO: Remove this function in v6.
// eslint-disable-next-line @typescript-eslint/naming-convention
export function experimental_sx() {
  throw /* minify-error */ new Error(
    'MUI: The `experimental_sx` has been moved to `theme.unstable_sx`.' +
      'For more details, see https://github.com/mui/material-ui/pull/35150.',
  );
}
export { default as createTheme } from './createTheme';
export { default as unstable_createMuiStrictModeTheme } from './createMuiStrictModeTheme';
export { getUnit as unstable_getUnit, toUnitless as unstable_toUnitless } from './cssUtils';
export { default as responsiveFontSizes } from './responsiveFontSizes';
export { default as createTransitions, duration, easing } from './createTransitions';
export { default as createColorScheme } from './createColorScheme';
export { default as useTheme } from './useTheme';
export { default as useThemeProps } from './useThemeProps';
export { default as styled } from './styled';
export { default as ThemeProvider } from './ThemeProvider';
export { StyledEngineProvider } from '@mui/system';
// Legacy styling shims were removed in favor of automation-first flows.
// Developers should migrate to the RusticUI styling macros, e.g. `rustic_ui_styled_engine::css_with_theme!`,
// or other supported styling approaches documented in `docs/mui-compatibility.md`.

export * from './ThemeProviderWithVars';
export { default as extendTheme } from './createThemeWithVars';
export { default as experimental_extendTheme } from './experimental_extendTheme'; // TODO: Remove in v7
export { default as getOverlayAlpha } from './getOverlayAlpha';
export { default as shouldSkipGeneratingVar } from './shouldSkipGeneratingVar';

// Private methods for creating parts of the theme
export { default as private_createTypography } from './createTypography';
export { default as private_createMixins } from './createMixins';
export { default as private_excludeVariablesFromRoot } from './excludeVariablesFromRoot';
