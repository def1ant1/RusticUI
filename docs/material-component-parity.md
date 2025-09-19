# Material Component Parity

_Last updated 2025-09-19T13:05:32.393450968+00:00 via `cargo xtask material-parity`._

## Coverage snapshot

- React exports analyzed: 146\n- `mui-material` coverage: 6 (4.1%)\n- `mui-headless` coverage: 1 (0.7%)\n
## Highest priority gaps

| Rank | Component | Source |
| --- | --- | --- |
| 1 | Accordion | `packages/mui-material/src/Accordion` |
| 2 | AccordionActions | `packages/mui-material/src/AccordionActions` |
| 3 | AccordionDetails | `packages/mui-material/src/AccordionDetails` |
| 4 | AccordionSummary | `packages/mui-material/src/AccordionSummary` |
| 5 | Alert | `packages/mui-material/src/Alert` |
| 6 | AlertTitle | `packages/mui-material/src/AlertTitle` |
| 7 | Autocomplete | `packages/mui-material/src/Autocomplete` |
| 8 | Avatar | `packages/mui-material/src/Avatar` |
| 9 | AvatarGroup | `packages/mui-material/src/AvatarGroup` |
| 10 | Backdrop | `packages/mui-material/src/Backdrop` |

## Machine-readable snapshot

```json
{
  "generated_at": "2025-09-19T13:05:32.393450968Z",
  "total_components": 146,
  "supported_in_material": 6,
  "supported_in_headless": 1,
  "material_coverage": 0.04109589,
  "headless_coverage": 0.006849315,
  "components": [
    {
      "name": "Accordion",
      "normalized": "accordion",
      "source": "packages/mui-material/src/Accordion",
      "declared_in": "packages/mui-material/src/Accordion/index.js"
    },
    {
      "name": "AccordionActions",
      "normalized": "accordion_actions",
      "source": "packages/mui-material/src/AccordionActions",
      "declared_in": "packages/mui-material/src/AccordionActions/index.js"
    },
    {
      "name": "AccordionDetails",
      "normalized": "accordion_details",
      "source": "packages/mui-material/src/AccordionDetails",
      "declared_in": "packages/mui-material/src/AccordionDetails/index.js"
    },
    {
      "name": "AccordionSummary",
      "normalized": "accordion_summary",
      "source": "packages/mui-material/src/AccordionSummary",
      "declared_in": "packages/mui-material/src/AccordionSummary/index.js"
    },
    {
      "name": "Alert",
      "normalized": "alert",
      "source": "packages/mui-material/src/Alert",
      "declared_in": "packages/mui-material/src/Alert/index.js"
    },
    {
      "name": "AlertTitle",
      "normalized": "alert_title",
      "source": "packages/mui-material/src/AlertTitle",
      "declared_in": "packages/mui-material/src/AlertTitle/index.js"
    },
    {
      "name": "AppBar",
      "normalized": "app_bar",
      "source": "packages/mui-material/src/AppBar",
      "declared_in": "packages/mui-material/src/AppBar/index.js"
    },
    {
      "name": "Autocomplete",
      "normalized": "autocomplete",
      "source": "packages/mui-material/src/Autocomplete",
      "declared_in": "packages/mui-material/src/Autocomplete/index.js"
    },
    {
      "name": "Avatar",
      "normalized": "avatar",
      "source": "packages/mui-material/src/Avatar",
      "declared_in": "packages/mui-material/src/Avatar/index.js"
    },
    {
      "name": "AvatarGroup",
      "normalized": "avatar_group",
      "source": "packages/mui-material/src/AvatarGroup",
      "declared_in": "packages/mui-material/src/AvatarGroup/index.js"
    },
    {
      "name": "Backdrop",
      "normalized": "backdrop",
      "source": "packages/mui-material/src/Backdrop",
      "declared_in": "packages/mui-material/src/Backdrop/index.js"
    },
    {
      "name": "Badge",
      "normalized": "badge",
      "source": "packages/mui-material/src/Badge",
      "declared_in": "packages/mui-material/src/Badge/index.js"
    },
    {
      "name": "BottomNavigation",
      "normalized": "bottom_navigation",
      "source": "packages/mui-material/src/BottomNavigation",
      "declared_in": "packages/mui-material/src/BottomNavigation/index.js"
    },
    {
      "name": "BottomNavigationAction",
      "normalized": "bottom_navigation_action",
      "source": "packages/mui-material/src/BottomNavigationAction",
      "declared_in": "packages/mui-material/src/BottomNavigationAction/index.js"
    },
    {
      "name": "Box",
      "normalized": "box",
      "source": "packages/mui-material/src/Box",
      "declared_in": "packages/mui-material/src/Box/index.js"
    },
    {
      "name": "Breadcrumbs",
      "normalized": "breadcrumbs",
      "source": "packages/mui-material/src/Breadcrumbs",
      "declared_in": "packages/mui-material/src/Breadcrumbs/index.js"
    },
    {
      "name": "Button",
      "normalized": "button",
      "source": "packages/mui-material/src/Button",
      "declared_in": "packages/mui-material/src/Button/index.js"
    },
    {
      "name": "ButtonBase",
      "normalized": "button_base",
      "source": "packages/mui-material/src/ButtonBase",
      "declared_in": "packages/mui-material/src/ButtonBase/index.js"
    },
    {
      "name": "ButtonGroup",
      "normalized": "button_group",
      "source": "packages/mui-material/src/ButtonGroup",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupButtonContext",
      "normalized": "button_group_button_context",
      "source": "packages/mui-material/src/ButtonGroupButtonContext",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupContext",
      "normalized": "button_group_context",
      "source": "packages/mui-material/src/ButtonGroupContext",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "Card",
      "normalized": "card",
      "source": "packages/mui-material/src/Card",
      "declared_in": "packages/mui-material/src/Card/index.js"
    },
    {
      "name": "CardActionArea",
      "normalized": "card_action_area",
      "source": "packages/mui-material/src/CardActionArea",
      "declared_in": "packages/mui-material/src/CardActionArea/index.js"
    },
    {
      "name": "CardActions",
      "normalized": "card_actions",
      "source": "packages/mui-material/src/CardActions",
      "declared_in": "packages/mui-material/src/CardActions/index.js"
    },
    {
      "name": "CardContent",
      "normalized": "card_content",
      "source": "packages/mui-material/src/CardContent",
      "declared_in": "packages/mui-material/src/CardContent/index.js"
    },
    {
      "name": "CardHeader",
      "normalized": "card_header",
      "source": "packages/mui-material/src/CardHeader",
      "declared_in": "packages/mui-material/src/CardHeader/index.js"
    },
    {
      "name": "CardMedia",
      "normalized": "card_media",
      "source": "packages/mui-material/src/CardMedia",
      "declared_in": "packages/mui-material/src/CardMedia/index.js"
    },
    {
      "name": "Checkbox",
      "normalized": "checkbox",
      "source": "packages/mui-material/src/Checkbox",
      "declared_in": "packages/mui-material/src/Checkbox/index.js"
    },
    {
      "name": "Chip",
      "normalized": "chip",
      "source": "packages/mui-material/src/Chip",
      "declared_in": "packages/mui-material/src/Chip/index.js"
    },
    {
      "name": "CircularProgress",
      "normalized": "circular_progress",
      "source": "packages/mui-material/src/CircularProgress",
      "declared_in": "packages/mui-material/src/CircularProgress/index.js"
    },
    {
      "name": "ClickAwayListener",
      "normalized": "click_away_listener",
      "source": "packages/mui-material/src/ClickAwayListener",
      "declared_in": "packages/mui-material/src/index.js"
    },
    {
      "name": "Collapse",
      "normalized": "collapse",
      "source": "packages/mui-material/src/Collapse",
      "declared_in": "packages/mui-material/src/Collapse/index.js"
    },
    {
      "name": "Container",
      "normalized": "container",
      "source": "packages/mui-material/src/Container",
      "declared_in": "packages/mui-material/src/Container/index.js"
    },
    {
      "name": "CssBaseline",
      "normalized": "css_baseline",
      "source": "packages/mui-material/src/CssBaseline",
      "declared_in": "packages/mui-material/src/CssBaseline/index.js"
    },
    {
      "name": "DefaultPropsProvider",
      "normalized": "default_props_provider",
      "source": "packages/mui-material/src/DefaultPropsProvider",
      "declared_in": "packages/mui-material/src/DefaultPropsProvider/index.ts"
    },
    {
      "name": "Dialog",
      "normalized": "dialog",
      "source": "packages/mui-material/src/Dialog",
      "declared_in": "packages/mui-material/src/Dialog/index.js"
    },
    {
      "name": "DialogActions",
      "normalized": "dialog_actions",
      "source": "packages/mui-material/src/DialogActions",
      "declared_in": "packages/mui-material/src/DialogActions/index.js"
    },
    {
      "name": "DialogContent",
      "normalized": "dialog_content",
      "source": "packages/mui-material/src/DialogContent",
      "declared_in": "packages/mui-material/src/DialogContent/index.js"
    },
    {
      "name": "DialogContentText",
      "normalized": "dialog_content_text",
      "source": "packages/mui-material/src/DialogContentText",
      "declared_in": "packages/mui-material/src/DialogContentText/index.js"
    },
    {
      "name": "DialogTitle",
      "normalized": "dialog_title",
      "source": "packages/mui-material/src/DialogTitle",
      "declared_in": "packages/mui-material/src/DialogTitle/index.js"
    },
    {
      "name": "Divider",
      "normalized": "divider",
      "source": "packages/mui-material/src/Divider",
      "declared_in": "packages/mui-material/src/Divider/index.js"
    },
    {
      "name": "Drawer",
      "normalized": "drawer",
      "source": "packages/mui-material/src/Drawer",
      "declared_in": "packages/mui-material/src/Drawer/index.js"
    },
    {
      "name": "Fab",
      "normalized": "fab",
      "source": "packages/mui-material/src/Fab",
      "declared_in": "packages/mui-material/src/Fab/index.js"
    },
    {
      "name": "Fade",
      "normalized": "fade",
      "source": "packages/mui-material/src/Fade",
      "declared_in": "packages/mui-material/src/Fade/index.js"
    },
    {
      "name": "FilledInput",
      "normalized": "filled_input",
      "source": "packages/mui-material/src/FilledInput",
      "declared_in": "packages/mui-material/src/FilledInput/index.js"
    },
    {
      "name": "FocusTrap",
      "normalized": "focus_trap",
      "source": "packages/mui-material/src/FocusTrap",
      "declared_in": "packages/mui-material/src/Unstable_TrapFocus/index.js"
    },
    {
      "name": "FormControl",
      "normalized": "form_control",
      "source": "packages/mui-material/src/FormControl",
      "declared_in": "packages/mui-material/src/FormControl/index.js"
    },
    {
      "name": "FormControlLabel",
      "normalized": "form_control_label",
      "source": "packages/mui-material/src/FormControlLabel",
      "declared_in": "packages/mui-material/src/FormControlLabel/index.js"
    },
    {
      "name": "FormGroup",
      "normalized": "form_group",
      "source": "packages/mui-material/src/FormGroup",
      "declared_in": "packages/mui-material/src/FormGroup/index.js"
    },
    {
      "name": "FormHelperText",
      "normalized": "form_helper_text",
      "source": "packages/mui-material/src/FormHelperText",
      "declared_in": "packages/mui-material/src/FormHelperText/index.js"
    },
    {
      "name": "FormLabel",
      "normalized": "form_label",
      "source": "packages/mui-material/src/FormLabel",
      "declared_in": "packages/mui-material/src/FormLabel/index.js"
    },
    {
      "name": "GlobalStyles",
      "normalized": "global_styles",
      "source": "packages/mui-material/src/GlobalStyles",
      "declared_in": "packages/mui-material/src/GlobalStyles/index.js"
    },
    {
      "name": "Grid",
      "normalized": "grid",
      "source": "packages/mui-material/src/Grid",
      "declared_in": "packages/mui-material/src/Grid/index.ts"
    },
    {
      "name": "GridLegacy",
      "normalized": "grid_legacy",
      "source": "packages/mui-material/src/GridLegacy",
      "declared_in": "packages/mui-material/src/GridLegacy/index.js"
    },
    {
      "name": "Grow",
      "normalized": "grow",
      "source": "packages/mui-material/src/Grow",
      "declared_in": "packages/mui-material/src/Grow/index.js"
    },
    {
      "name": "Icon",
      "normalized": "icon",
      "source": "packages/mui-material/src/Icon",
      "declared_in": "packages/mui-material/src/Icon/index.js"
    },
    {
      "name": "IconButton",
      "normalized": "icon_button",
      "source": "packages/mui-material/src/IconButton",
      "declared_in": "packages/mui-material/src/IconButton/index.js"
    },
    {
      "name": "ImageList",
      "normalized": "image_list",
      "source": "packages/mui-material/src/ImageList",
      "declared_in": "packages/mui-material/src/ImageList/index.js"
    },
    {
      "name": "ImageListItem",
      "normalized": "image_list_item",
      "source": "packages/mui-material/src/ImageListItem",
      "declared_in": "packages/mui-material/src/ImageListItem/index.js"
    },
    {
      "name": "ImageListItemBar",
      "normalized": "image_list_item_bar",
      "source": "packages/mui-material/src/ImageListItemBar",
      "declared_in": "packages/mui-material/src/ImageListItemBar/index.js"
    },
    {
      "name": "InitColorSchemeScript",
      "normalized": "init_color_scheme_script",
      "source": "packages/mui-material/src/InitColorSchemeScript",
      "declared_in": "packages/mui-material/src/InitColorSchemeScript/index.ts"
    },
    {
      "name": "Input",
      "normalized": "input",
      "source": "packages/mui-material/src/Input",
      "declared_in": "packages/mui-material/src/Input/index.js"
    },
    {
      "name": "InputAdornment",
      "normalized": "input_adornment",
      "source": "packages/mui-material/src/InputAdornment",
      "declared_in": "packages/mui-material/src/InputAdornment/index.js"
    },
    {
      "name": "InputBase",
      "normalized": "input_base",
      "source": "packages/mui-material/src/InputBase",
      "declared_in": "packages/mui-material/src/InputBase/index.js"
    },
    {
      "name": "InputLabel",
      "normalized": "input_label",
      "source": "packages/mui-material/src/InputLabel",
      "declared_in": "packages/mui-material/src/InputLabel/index.js"
    },
    {
      "name": "LinearProgress",
      "normalized": "linear_progress",
      "source": "packages/mui-material/src/LinearProgress",
      "declared_in": "packages/mui-material/src/LinearProgress/index.js"
    },
    {
      "name": "Link",
      "normalized": "link",
      "source": "packages/mui-material/src/Link",
      "declared_in": "packages/mui-material/src/Link/index.js"
    },
    {
      "name": "List",
      "normalized": "list",
      "source": "packages/mui-material/src/List",
      "declared_in": "packages/mui-material/src/List/index.js"
    },
    {
      "name": "ListItem",
      "normalized": "list_item",
      "source": "packages/mui-material/src/ListItem",
      "declared_in": "packages/mui-material/src/ListItem/index.js"
    },
    {
      "name": "ListItemAvatar",
      "normalized": "list_item_avatar",
      "source": "packages/mui-material/src/ListItemAvatar",
      "declared_in": "packages/mui-material/src/ListItemAvatar/index.js"
    },
    {
      "name": "ListItemButton",
      "normalized": "list_item_button",
      "source": "packages/mui-material/src/ListItemButton",
      "declared_in": "packages/mui-material/src/ListItemButton/index.js"
    },
    {
      "name": "ListItemIcon",
      "normalized": "list_item_icon",
      "source": "packages/mui-material/src/ListItemIcon",
      "declared_in": "packages/mui-material/src/ListItemIcon/index.js"
    },
    {
      "name": "ListItemSecondaryAction",
      "normalized": "list_item_secondary_action",
      "source": "packages/mui-material/src/ListItemSecondaryAction",
      "declared_in": "packages/mui-material/src/ListItemSecondaryAction/index.js"
    },
    {
      "name": "ListItemText",
      "normalized": "list_item_text",
      "source": "packages/mui-material/src/ListItemText",
      "declared_in": "packages/mui-material/src/ListItemText/index.js"
    },
    {
      "name": "ListSubheader",
      "normalized": "list_subheader",
      "source": "packages/mui-material/src/ListSubheader",
      "declared_in": "packages/mui-material/src/ListSubheader/index.js"
    },
    {
      "name": "Menu",
      "normalized": "menu",
      "source": "packages/mui-material/src/Menu",
      "declared_in": "packages/mui-material/src/Menu/index.js"
    },
    {
      "name": "MenuItem",
      "normalized": "menu_item",
      "source": "packages/mui-material/src/MenuItem",
      "declared_in": "packages/mui-material/src/MenuItem/index.js"
    },
    {
      "name": "MenuList",
      "normalized": "menu_list",
      "source": "packages/mui-material/src/MenuList",
      "declared_in": "packages/mui-material/src/MenuList/index.js"
    },
    {
      "name": "MobileStepper",
      "normalized": "mobile_stepper",
      "source": "packages/mui-material/src/MobileStepper",
      "declared_in": "packages/mui-material/src/MobileStepper/index.js"
    },
    {
      "name": "Modal",
      "normalized": "modal",
      "source": "packages/mui-material/src/Modal",
      "declared_in": "packages/mui-material/src/Modal/index.js"
    },
    {
      "name": "NativeSelect",
      "normalized": "native_select",
      "source": "packages/mui-material/src/NativeSelect",
      "declared_in": "packages/mui-material/src/NativeSelect/index.js"
    },
    {
      "name": "NoSsr",
      "normalized": "no_ssr",
      "source": "packages/mui-material/src/NoSsr",
      "declared_in": "packages/mui-material/src/NoSsr/index.js"
    },
    {
      "name": "OutlinedInput",
      "normalized": "outlined_input",
      "source": "packages/mui-material/src/OutlinedInput",
      "declared_in": "packages/mui-material/src/OutlinedInput/index.js"
    },
    {
      "name": "Pagination",
      "normalized": "pagination",
      "source": "packages/mui-material/src/Pagination",
      "declared_in": "packages/mui-material/src/Pagination/index.js"
    },
    {
      "name": "PaginationItem",
      "normalized": "pagination_item",
      "source": "packages/mui-material/src/PaginationItem",
      "declared_in": "packages/mui-material/src/PaginationItem/index.js"
    },
    {
      "name": "Paper",
      "normalized": "paper",
      "source": "packages/mui-material/src/Paper",
      "declared_in": "packages/mui-material/src/Paper/index.js"
    },
    {
      "name": "PigmentContainer",
      "normalized": "pigment_container",
      "source": "packages/mui-material/src/PigmentContainer",
      "declared_in": "packages/mui-material/src/PigmentContainer/index.ts"
    },
    {
      "name": "PigmentGrid",
      "normalized": "pigment_grid",
      "source": "packages/mui-material/src/PigmentGrid",
      "declared_in": "packages/mui-material/src/PigmentGrid/index.ts"
    },
    {
      "name": "PigmentStack",
      "normalized": "pigment_stack",
      "source": "packages/mui-material/src/PigmentStack",
      "declared_in": "packages/mui-material/src/PigmentStack/index.ts"
    },
    {
      "name": "Popover",
      "normalized": "popover",
      "source": "packages/mui-material/src/Popover",
      "declared_in": "packages/mui-material/src/Popover/index.js"
    },
    {
      "name": "Popper",
      "normalized": "popper",
      "source": "packages/mui-material/src/Popper",
      "declared_in": "packages/mui-material/src/Popper/index.js"
    },
    {
      "name": "Portal",
      "normalized": "portal",
      "source": "packages/mui-material/src/Portal",
      "declared_in": "packages/mui-material/src/Portal/index.js"
    },
    {
      "name": "Radio",
      "normalized": "radio",
      "source": "packages/mui-material/src/Radio",
      "declared_in": "packages/mui-material/src/Radio/index.js"
    },
    {
      "name": "RadioGroup",
      "normalized": "radio_group",
      "source": "packages/mui-material/src/RadioGroup",
      "declared_in": "packages/mui-material/src/RadioGroup/index.js"
    },
    {
      "name": "Rating",
      "normalized": "rating",
      "source": "packages/mui-material/src/Rating",
      "declared_in": "packages/mui-material/src/Rating/index.js"
    },
    {
      "name": "ScopedCssBaseline",
      "normalized": "scoped_css_baseline",
      "source": "packages/mui-material/src/ScopedCssBaseline",
      "declared_in": "packages/mui-material/src/ScopedCssBaseline/index.js"
    },
    {
      "name": "Select",
      "normalized": "select",
      "source": "packages/mui-material/src/Select",
      "declared_in": "packages/mui-material/src/Select/index.js"
    },
    {
      "name": "Skeleton",
      "normalized": "skeleton",
      "source": "packages/mui-material/src/Skeleton",
      "declared_in": "packages/mui-material/src/Skeleton/index.js"
    },
    {
      "name": "Slide",
      "normalized": "slide",
      "source": "packages/mui-material/src/Slide",
      "declared_in": "packages/mui-material/src/Slide/index.js"
    },
    {
      "name": "Slider",
      "normalized": "slider",
      "source": "packages/mui-material/src/Slider",
      "declared_in": "packages/mui-material/src/Slider/index.js"
    },
    {
      "name": "Snackbar",
      "normalized": "snackbar",
      "source": "packages/mui-material/src/Snackbar",
      "declared_in": "packages/mui-material/src/Snackbar/index.js"
    },
    {
      "name": "SnackbarContent",
      "normalized": "snackbar_content",
      "source": "packages/mui-material/src/SnackbarContent",
      "declared_in": "packages/mui-material/src/SnackbarContent/index.js"
    },
    {
      "name": "SpeedDial",
      "normalized": "speed_dial",
      "source": "packages/mui-material/src/SpeedDial",
      "declared_in": "packages/mui-material/src/SpeedDial/index.js"
    },
    {
      "name": "SpeedDialAction",
      "normalized": "speed_dial_action",
      "source": "packages/mui-material/src/SpeedDialAction",
      "declared_in": "packages/mui-material/src/SpeedDialAction/index.js"
    },
    {
      "name": "SpeedDialIcon",
      "normalized": "speed_dial_icon",
      "source": "packages/mui-material/src/SpeedDialIcon",
      "declared_in": "packages/mui-material/src/SpeedDialIcon/index.js"
    },
    {
      "name": "Stack",
      "normalized": "stack",
      "source": "packages/mui-material/src/Stack",
      "declared_in": "packages/mui-material/src/Stack/index.js"
    },
    {
      "name": "Step",
      "normalized": "step",
      "source": "packages/mui-material/src/Step",
      "declared_in": "packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepButton",
      "normalized": "step_button",
      "source": "packages/mui-material/src/StepButton",
      "declared_in": "packages/mui-material/src/StepButton/index.js"
    },
    {
      "name": "StepConnector",
      "normalized": "step_connector",
      "source": "packages/mui-material/src/StepConnector",
      "declared_in": "packages/mui-material/src/StepConnector/index.js"
    },
    {
      "name": "StepContent",
      "normalized": "step_content",
      "source": "packages/mui-material/src/StepContent",
      "declared_in": "packages/mui-material/src/StepContent/index.js"
    },
    {
      "name": "StepContext",
      "normalized": "step_context",
      "source": "packages/mui-material/src/StepContext",
      "declared_in": "packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepIcon",
      "normalized": "step_icon",
      "source": "packages/mui-material/src/StepIcon",
      "declared_in": "packages/mui-material/src/StepIcon/index.js"
    },
    {
      "name": "StepLabel",
      "normalized": "step_label",
      "source": "packages/mui-material/src/StepLabel",
      "declared_in": "packages/mui-material/src/StepLabel/index.js"
    },
    {
      "name": "Stepper",
      "normalized": "stepper",
      "source": "packages/mui-material/src/Stepper",
      "declared_in": "packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "StepperContext",
      "normalized": "stepper_context",
      "source": "packages/mui-material/src/StepperContext",
      "declared_in": "packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "SvgIcon",
      "normalized": "svg_icon",
      "source": "packages/mui-material/src/SvgIcon",
      "declared_in": "packages/mui-material/src/SvgIcon/index.js"
    },
    {
      "name": "SwipeableDrawer",
      "normalized": "swipeable_drawer",
      "source": "packages/mui-material/src/SwipeableDrawer",
      "declared_in": "packages/mui-material/src/SwipeableDrawer/index.js"
    },
    {
      "name": "Switch",
      "normalized": "switch",
      "source": "packages/mui-material/src/Switch",
      "declared_in": "packages/mui-material/src/Switch/index.js"
    },
    {
      "name": "Tab",
      "normalized": "tab",
      "source": "packages/mui-material/src/Tab",
      "declared_in": "packages/mui-material/src/Tab/index.js"
    },
    {
      "name": "TabScrollButton",
      "normalized": "tab_scroll_button",
      "source": "packages/mui-material/src/TabScrollButton",
      "declared_in": "packages/mui-material/src/TabScrollButton/index.js"
    },
    {
      "name": "Table",
      "normalized": "table",
      "source": "packages/mui-material/src/Table",
      "declared_in": "packages/mui-material/src/Table/index.js"
    },
    {
      "name": "TableBody",
      "normalized": "table_body",
      "source": "packages/mui-material/src/TableBody",
      "declared_in": "packages/mui-material/src/TableBody/index.js"
    },
    {
      "name": "TableCell",
      "normalized": "table_cell",
      "source": "packages/mui-material/src/TableCell",
      "declared_in": "packages/mui-material/src/TableCell/index.js"
    },
    {
      "name": "TableContainer",
      "normalized": "table_container",
      "source": "packages/mui-material/src/TableContainer",
      "declared_in": "packages/mui-material/src/TableContainer/index.js"
    },
    {
      "name": "TableFooter",
      "normalized": "table_footer",
      "source": "packages/mui-material/src/TableFooter",
      "declared_in": "packages/mui-material/src/TableFooter/index.js"
    },
    {
      "name": "TableHead",
      "normalized": "table_head",
      "source": "packages/mui-material/src/TableHead",
      "declared_in": "packages/mui-material/src/TableHead/index.js"
    },
    {
      "name": "TablePagination",
      "normalized": "table_pagination",
      "source": "packages/mui-material/src/TablePagination",
      "declared_in": "packages/mui-material/src/TablePagination/index.js"
    },
    {
      "name": "TablePaginationActions",
      "normalized": "table_pagination_actions",
      "source": "packages/mui-material/src/TablePaginationActions",
      "declared_in": "packages/mui-material/src/TablePaginationActions/index.js"
    },
    {
      "name": "TableRow",
      "normalized": "table_row",
      "source": "packages/mui-material/src/TableRow",
      "declared_in": "packages/mui-material/src/TableRow/index.js"
    },
    {
      "name": "TableSortLabel",
      "normalized": "table_sort_label",
      "source": "packages/mui-material/src/TableSortLabel",
      "declared_in": "packages/mui-material/src/TableSortLabel/index.js"
    },
    {
      "name": "Tabs",
      "normalized": "tabs",
      "source": "packages/mui-material/src/Tabs",
      "declared_in": "packages/mui-material/src/Tabs/index.js"
    },
    {
      "name": "TextField",
      "normalized": "text_field",
      "source": "packages/mui-material/src/TextField",
      "declared_in": "packages/mui-material/src/TextField/index.js"
    },
    {
      "name": "TextareaAutosize",
      "normalized": "textarea_autosize",
      "source": "packages/mui-material/src/TextareaAutosize",
      "declared_in": "packages/mui-material/src/TextareaAutosize/index.js"
    },
    {
      "name": "THEME_ID",
      "normalized": "theme_id",
      "source": "packages/mui-material/src/identifier",
      "declared_in": "packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ThemeProvider",
      "normalized": "theme_provider",
      "source": "packages/mui-material/src/ThemeProvider",
      "declared_in": "packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ToggleButton",
      "normalized": "toggle_button",
      "source": "packages/mui-material/src/ToggleButton",
      "declared_in": "packages/mui-material/src/ToggleButton/index.js"
    },
    {
      "name": "ToggleButtonGroup",
      "normalized": "toggle_button_group",
      "source": "packages/mui-material/src/ToggleButtonGroup",
      "declared_in": "packages/mui-material/src/ToggleButtonGroup/index.js"
    },
    {
      "name": "Toolbar",
      "normalized": "toolbar",
      "source": "packages/mui-material/src/Toolbar",
      "declared_in": "packages/mui-material/src/Toolbar/index.js"
    },
    {
      "name": "Tooltip",
      "normalized": "tooltip",
      "source": "packages/mui-material/src/Tooltip",
      "declared_in": "packages/mui-material/src/Tooltip/index.js"
    },
    {
      "name": "Typography",
      "normalized": "typography",
      "source": "packages/mui-material/src/Typography",
      "declared_in": "packages/mui-material/src/Typography/index.js"
    },
    {
      "name": "Unstable_TrapFocus",
      "normalized": "unstable_trap_focus",
      "source": "packages/mui-material/src/Unstable_TrapFocus",
      "declared_in": "packages/mui-material/src/index.js"
    },
    {
      "name": "UseAutocomplete",
      "normalized": "use_autocomplete",
      "source": "packages/mui-material/src/useAutocomplete",
      "declared_in": "packages/mui-material/src/useAutocomplete/index.js"
    },
    {
      "name": "UseLazyRipple",
      "normalized": "use_lazy_ripple",
      "source": "packages/mui-material/src/useLazyRipple",
      "declared_in": "packages/mui-material/src/useLazyRipple/index.ts"
    },
    {
      "name": "UsePagination",
      "normalized": "use_pagination",
      "source": "packages/mui-material/src/usePagination",
      "declared_in": "packages/mui-material/src/usePagination/index.js"
    },
    {
      "name": "UseScrollTrigger",
      "normalized": "use_scroll_trigger",
      "source": "packages/mui-material/src/useScrollTrigger",
      "declared_in": "packages/mui-material/src/useScrollTrigger/index.js"
    },
    {
      "name": "Zoom",
      "normalized": "zoom",
      "source": "packages/mui-material/src/Zoom",
      "declared_in": "packages/mui-material/src/Zoom/index.js"
    }
  ],
  "missing_from_material": [
    {
      "name": "Accordion",
      "normalized": "accordion",
      "source": "packages/mui-material/src/Accordion",
      "declared_in": "packages/mui-material/src/Accordion/index.js"
    },
    {
      "name": "AccordionActions",
      "normalized": "accordion_actions",
      "source": "packages/mui-material/src/AccordionActions",
      "declared_in": "packages/mui-material/src/AccordionActions/index.js"
    },
    {
      "name": "AccordionDetails",
      "normalized": "accordion_details",
      "source": "packages/mui-material/src/AccordionDetails",
      "declared_in": "packages/mui-material/src/AccordionDetails/index.js"
    },
    {
      "name": "AccordionSummary",
      "normalized": "accordion_summary",
      "source": "packages/mui-material/src/AccordionSummary",
      "declared_in": "packages/mui-material/src/AccordionSummary/index.js"
    },
    {
      "name": "Alert",
      "normalized": "alert",
      "source": "packages/mui-material/src/Alert",
      "declared_in": "packages/mui-material/src/Alert/index.js"
    },
    {
      "name": "AlertTitle",
      "normalized": "alert_title",
      "source": "packages/mui-material/src/AlertTitle",
      "declared_in": "packages/mui-material/src/AlertTitle/index.js"
    },
    {
      "name": "Autocomplete",
      "normalized": "autocomplete",
      "source": "packages/mui-material/src/Autocomplete",
      "declared_in": "packages/mui-material/src/Autocomplete/index.js"
    },
    {
      "name": "Avatar",
      "normalized": "avatar",
      "source": "packages/mui-material/src/Avatar",
      "declared_in": "packages/mui-material/src/Avatar/index.js"
    },
    {
      "name": "AvatarGroup",
      "normalized": "avatar_group",
      "source": "packages/mui-material/src/AvatarGroup",
      "declared_in": "packages/mui-material/src/AvatarGroup/index.js"
    },
    {
      "name": "Backdrop",
      "normalized": "backdrop",
      "source": "packages/mui-material/src/Backdrop",
      "declared_in": "packages/mui-material/src/Backdrop/index.js"
    },
    {
      "name": "Badge",
      "normalized": "badge",
      "source": "packages/mui-material/src/Badge",
      "declared_in": "packages/mui-material/src/Badge/index.js"
    },
    {
      "name": "BottomNavigation",
      "normalized": "bottom_navigation",
      "source": "packages/mui-material/src/BottomNavigation",
      "declared_in": "packages/mui-material/src/BottomNavigation/index.js"
    },
    {
      "name": "BottomNavigationAction",
      "normalized": "bottom_navigation_action",
      "source": "packages/mui-material/src/BottomNavigationAction",
      "declared_in": "packages/mui-material/src/BottomNavigationAction/index.js"
    },
    {
      "name": "Box",
      "normalized": "box",
      "source": "packages/mui-material/src/Box",
      "declared_in": "packages/mui-material/src/Box/index.js"
    },
    {
      "name": "Breadcrumbs",
      "normalized": "breadcrumbs",
      "source": "packages/mui-material/src/Breadcrumbs",
      "declared_in": "packages/mui-material/src/Breadcrumbs/index.js"
    },
    {
      "name": "ButtonBase",
      "normalized": "button_base",
      "source": "packages/mui-material/src/ButtonBase",
      "declared_in": "packages/mui-material/src/ButtonBase/index.js"
    },
    {
      "name": "ButtonGroup",
      "normalized": "button_group",
      "source": "packages/mui-material/src/ButtonGroup",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupButtonContext",
      "normalized": "button_group_button_context",
      "source": "packages/mui-material/src/ButtonGroupButtonContext",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupContext",
      "normalized": "button_group_context",
      "source": "packages/mui-material/src/ButtonGroupContext",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "CardActionArea",
      "normalized": "card_action_area",
      "source": "packages/mui-material/src/CardActionArea",
      "declared_in": "packages/mui-material/src/CardActionArea/index.js"
    },
    {
      "name": "CardActions",
      "normalized": "card_actions",
      "source": "packages/mui-material/src/CardActions",
      "declared_in": "packages/mui-material/src/CardActions/index.js"
    },
    {
      "name": "CardContent",
      "normalized": "card_content",
      "source": "packages/mui-material/src/CardContent",
      "declared_in": "packages/mui-material/src/CardContent/index.js"
    },
    {
      "name": "CardHeader",
      "normalized": "card_header",
      "source": "packages/mui-material/src/CardHeader",
      "declared_in": "packages/mui-material/src/CardHeader/index.js"
    },
    {
      "name": "CardMedia",
      "normalized": "card_media",
      "source": "packages/mui-material/src/CardMedia",
      "declared_in": "packages/mui-material/src/CardMedia/index.js"
    },
    {
      "name": "Checkbox",
      "normalized": "checkbox",
      "source": "packages/mui-material/src/Checkbox",
      "declared_in": "packages/mui-material/src/Checkbox/index.js"
    },
    {
      "name": "Chip",
      "normalized": "chip",
      "source": "packages/mui-material/src/Chip",
      "declared_in": "packages/mui-material/src/Chip/index.js"
    },
    {
      "name": "CircularProgress",
      "normalized": "circular_progress",
      "source": "packages/mui-material/src/CircularProgress",
      "declared_in": "packages/mui-material/src/CircularProgress/index.js"
    },
    {
      "name": "ClickAwayListener",
      "normalized": "click_away_listener",
      "source": "packages/mui-material/src/ClickAwayListener",
      "declared_in": "packages/mui-material/src/index.js"
    },
    {
      "name": "Collapse",
      "normalized": "collapse",
      "source": "packages/mui-material/src/Collapse",
      "declared_in": "packages/mui-material/src/Collapse/index.js"
    },
    {
      "name": "Container",
      "normalized": "container",
      "source": "packages/mui-material/src/Container",
      "declared_in": "packages/mui-material/src/Container/index.js"
    },
    {
      "name": "CssBaseline",
      "normalized": "css_baseline",
      "source": "packages/mui-material/src/CssBaseline",
      "declared_in": "packages/mui-material/src/CssBaseline/index.js"
    },
    {
      "name": "DefaultPropsProvider",
      "normalized": "default_props_provider",
      "source": "packages/mui-material/src/DefaultPropsProvider",
      "declared_in": "packages/mui-material/src/DefaultPropsProvider/index.ts"
    },
    {
      "name": "DialogActions",
      "normalized": "dialog_actions",
      "source": "packages/mui-material/src/DialogActions",
      "declared_in": "packages/mui-material/src/DialogActions/index.js"
    },
    {
      "name": "DialogContent",
      "normalized": "dialog_content",
      "source": "packages/mui-material/src/DialogContent",
      "declared_in": "packages/mui-material/src/DialogContent/index.js"
    },
    {
      "name": "DialogContentText",
      "normalized": "dialog_content_text",
      "source": "packages/mui-material/src/DialogContentText",
      "declared_in": "packages/mui-material/src/DialogContentText/index.js"
    },
    {
      "name": "DialogTitle",
      "normalized": "dialog_title",
      "source": "packages/mui-material/src/DialogTitle",
      "declared_in": "packages/mui-material/src/DialogTitle/index.js"
    },
    {
      "name": "Divider",
      "normalized": "divider",
      "source": "packages/mui-material/src/Divider",
      "declared_in": "packages/mui-material/src/Divider/index.js"
    },
    {
      "name": "Drawer",
      "normalized": "drawer",
      "source": "packages/mui-material/src/Drawer",
      "declared_in": "packages/mui-material/src/Drawer/index.js"
    },
    {
      "name": "Fab",
      "normalized": "fab",
      "source": "packages/mui-material/src/Fab",
      "declared_in": "packages/mui-material/src/Fab/index.js"
    },
    {
      "name": "Fade",
      "normalized": "fade",
      "source": "packages/mui-material/src/Fade",
      "declared_in": "packages/mui-material/src/Fade/index.js"
    },
    {
      "name": "FilledInput",
      "normalized": "filled_input",
      "source": "packages/mui-material/src/FilledInput",
      "declared_in": "packages/mui-material/src/FilledInput/index.js"
    },
    {
      "name": "FocusTrap",
      "normalized": "focus_trap",
      "source": "packages/mui-material/src/FocusTrap",
      "declared_in": "packages/mui-material/src/Unstable_TrapFocus/index.js"
    },
    {
      "name": "FormControl",
      "normalized": "form_control",
      "source": "packages/mui-material/src/FormControl",
      "declared_in": "packages/mui-material/src/FormControl/index.js"
    },
    {
      "name": "FormControlLabel",
      "normalized": "form_control_label",
      "source": "packages/mui-material/src/FormControlLabel",
      "declared_in": "packages/mui-material/src/FormControlLabel/index.js"
    },
    {
      "name": "FormGroup",
      "normalized": "form_group",
      "source": "packages/mui-material/src/FormGroup",
      "declared_in": "packages/mui-material/src/FormGroup/index.js"
    },
    {
      "name": "FormHelperText",
      "normalized": "form_helper_text",
      "source": "packages/mui-material/src/FormHelperText",
      "declared_in": "packages/mui-material/src/FormHelperText/index.js"
    },
    {
      "name": "FormLabel",
      "normalized": "form_label",
      "source": "packages/mui-material/src/FormLabel",
      "declared_in": "packages/mui-material/src/FormLabel/index.js"
    },
    {
      "name": "GlobalStyles",
      "normalized": "global_styles",
      "source": "packages/mui-material/src/GlobalStyles",
      "declared_in": "packages/mui-material/src/GlobalStyles/index.js"
    },
    {
      "name": "Grid",
      "normalized": "grid",
      "source": "packages/mui-material/src/Grid",
      "declared_in": "packages/mui-material/src/Grid/index.ts"
    },
    {
      "name": "GridLegacy",
      "normalized": "grid_legacy",
      "source": "packages/mui-material/src/GridLegacy",
      "declared_in": "packages/mui-material/src/GridLegacy/index.js"
    },
    {
      "name": "Grow",
      "normalized": "grow",
      "source": "packages/mui-material/src/Grow",
      "declared_in": "packages/mui-material/src/Grow/index.js"
    },
    {
      "name": "Icon",
      "normalized": "icon",
      "source": "packages/mui-material/src/Icon",
      "declared_in": "packages/mui-material/src/Icon/index.js"
    },
    {
      "name": "IconButton",
      "normalized": "icon_button",
      "source": "packages/mui-material/src/IconButton",
      "declared_in": "packages/mui-material/src/IconButton/index.js"
    },
    {
      "name": "ImageList",
      "normalized": "image_list",
      "source": "packages/mui-material/src/ImageList",
      "declared_in": "packages/mui-material/src/ImageList/index.js"
    },
    {
      "name": "ImageListItem",
      "normalized": "image_list_item",
      "source": "packages/mui-material/src/ImageListItem",
      "declared_in": "packages/mui-material/src/ImageListItem/index.js"
    },
    {
      "name": "ImageListItemBar",
      "normalized": "image_list_item_bar",
      "source": "packages/mui-material/src/ImageListItemBar",
      "declared_in": "packages/mui-material/src/ImageListItemBar/index.js"
    },
    {
      "name": "InitColorSchemeScript",
      "normalized": "init_color_scheme_script",
      "source": "packages/mui-material/src/InitColorSchemeScript",
      "declared_in": "packages/mui-material/src/InitColorSchemeScript/index.ts"
    },
    {
      "name": "Input",
      "normalized": "input",
      "source": "packages/mui-material/src/Input",
      "declared_in": "packages/mui-material/src/Input/index.js"
    },
    {
      "name": "InputAdornment",
      "normalized": "input_adornment",
      "source": "packages/mui-material/src/InputAdornment",
      "declared_in": "packages/mui-material/src/InputAdornment/index.js"
    },
    {
      "name": "InputBase",
      "normalized": "input_base",
      "source": "packages/mui-material/src/InputBase",
      "declared_in": "packages/mui-material/src/InputBase/index.js"
    },
    {
      "name": "InputLabel",
      "normalized": "input_label",
      "source": "packages/mui-material/src/InputLabel",
      "declared_in": "packages/mui-material/src/InputLabel/index.js"
    },
    {
      "name": "LinearProgress",
      "normalized": "linear_progress",
      "source": "packages/mui-material/src/LinearProgress",
      "declared_in": "packages/mui-material/src/LinearProgress/index.js"
    },
    {
      "name": "Link",
      "normalized": "link",
      "source": "packages/mui-material/src/Link",
      "declared_in": "packages/mui-material/src/Link/index.js"
    },
    {
      "name": "List",
      "normalized": "list",
      "source": "packages/mui-material/src/List",
      "declared_in": "packages/mui-material/src/List/index.js"
    },
    {
      "name": "ListItem",
      "normalized": "list_item",
      "source": "packages/mui-material/src/ListItem",
      "declared_in": "packages/mui-material/src/ListItem/index.js"
    },
    {
      "name": "ListItemAvatar",
      "normalized": "list_item_avatar",
      "source": "packages/mui-material/src/ListItemAvatar",
      "declared_in": "packages/mui-material/src/ListItemAvatar/index.js"
    },
    {
      "name": "ListItemButton",
      "normalized": "list_item_button",
      "source": "packages/mui-material/src/ListItemButton",
      "declared_in": "packages/mui-material/src/ListItemButton/index.js"
    },
    {
      "name": "ListItemIcon",
      "normalized": "list_item_icon",
      "source": "packages/mui-material/src/ListItemIcon",
      "declared_in": "packages/mui-material/src/ListItemIcon/index.js"
    },
    {
      "name": "ListItemSecondaryAction",
      "normalized": "list_item_secondary_action",
      "source": "packages/mui-material/src/ListItemSecondaryAction",
      "declared_in": "packages/mui-material/src/ListItemSecondaryAction/index.js"
    },
    {
      "name": "ListItemText",
      "normalized": "list_item_text",
      "source": "packages/mui-material/src/ListItemText",
      "declared_in": "packages/mui-material/src/ListItemText/index.js"
    },
    {
      "name": "ListSubheader",
      "normalized": "list_subheader",
      "source": "packages/mui-material/src/ListSubheader",
      "declared_in": "packages/mui-material/src/ListSubheader/index.js"
    },
    {
      "name": "Menu",
      "normalized": "menu",
      "source": "packages/mui-material/src/Menu",
      "declared_in": "packages/mui-material/src/Menu/index.js"
    },
    {
      "name": "MenuItem",
      "normalized": "menu_item",
      "source": "packages/mui-material/src/MenuItem",
      "declared_in": "packages/mui-material/src/MenuItem/index.js"
    },
    {
      "name": "MenuList",
      "normalized": "menu_list",
      "source": "packages/mui-material/src/MenuList",
      "declared_in": "packages/mui-material/src/MenuList/index.js"
    },
    {
      "name": "MobileStepper",
      "normalized": "mobile_stepper",
      "source": "packages/mui-material/src/MobileStepper",
      "declared_in": "packages/mui-material/src/MobileStepper/index.js"
    },
    {
      "name": "Modal",
      "normalized": "modal",
      "source": "packages/mui-material/src/Modal",
      "declared_in": "packages/mui-material/src/Modal/index.js"
    },
    {
      "name": "NativeSelect",
      "normalized": "native_select",
      "source": "packages/mui-material/src/NativeSelect",
      "declared_in": "packages/mui-material/src/NativeSelect/index.js"
    },
    {
      "name": "NoSsr",
      "normalized": "no_ssr",
      "source": "packages/mui-material/src/NoSsr",
      "declared_in": "packages/mui-material/src/NoSsr/index.js"
    },
    {
      "name": "OutlinedInput",
      "normalized": "outlined_input",
      "source": "packages/mui-material/src/OutlinedInput",
      "declared_in": "packages/mui-material/src/OutlinedInput/index.js"
    },
    {
      "name": "Pagination",
      "normalized": "pagination",
      "source": "packages/mui-material/src/Pagination",
      "declared_in": "packages/mui-material/src/Pagination/index.js"
    },
    {
      "name": "PaginationItem",
      "normalized": "pagination_item",
      "source": "packages/mui-material/src/PaginationItem",
      "declared_in": "packages/mui-material/src/PaginationItem/index.js"
    },
    {
      "name": "Paper",
      "normalized": "paper",
      "source": "packages/mui-material/src/Paper",
      "declared_in": "packages/mui-material/src/Paper/index.js"
    },
    {
      "name": "PigmentContainer",
      "normalized": "pigment_container",
      "source": "packages/mui-material/src/PigmentContainer",
      "declared_in": "packages/mui-material/src/PigmentContainer/index.ts"
    },
    {
      "name": "PigmentGrid",
      "normalized": "pigment_grid",
      "source": "packages/mui-material/src/PigmentGrid",
      "declared_in": "packages/mui-material/src/PigmentGrid/index.ts"
    },
    {
      "name": "PigmentStack",
      "normalized": "pigment_stack",
      "source": "packages/mui-material/src/PigmentStack",
      "declared_in": "packages/mui-material/src/PigmentStack/index.ts"
    },
    {
      "name": "Popover",
      "normalized": "popover",
      "source": "packages/mui-material/src/Popover",
      "declared_in": "packages/mui-material/src/Popover/index.js"
    },
    {
      "name": "Popper",
      "normalized": "popper",
      "source": "packages/mui-material/src/Popper",
      "declared_in": "packages/mui-material/src/Popper/index.js"
    },
    {
      "name": "Portal",
      "normalized": "portal",
      "source": "packages/mui-material/src/Portal",
      "declared_in": "packages/mui-material/src/Portal/index.js"
    },
    {
      "name": "Radio",
      "normalized": "radio",
      "source": "packages/mui-material/src/Radio",
      "declared_in": "packages/mui-material/src/Radio/index.js"
    },
    {
      "name": "RadioGroup",
      "normalized": "radio_group",
      "source": "packages/mui-material/src/RadioGroup",
      "declared_in": "packages/mui-material/src/RadioGroup/index.js"
    },
    {
      "name": "Rating",
      "normalized": "rating",
      "source": "packages/mui-material/src/Rating",
      "declared_in": "packages/mui-material/src/Rating/index.js"
    },
    {
      "name": "ScopedCssBaseline",
      "normalized": "scoped_css_baseline",
      "source": "packages/mui-material/src/ScopedCssBaseline",
      "declared_in": "packages/mui-material/src/ScopedCssBaseline/index.js"
    },
    {
      "name": "Select",
      "normalized": "select",
      "source": "packages/mui-material/src/Select",
      "declared_in": "packages/mui-material/src/Select/index.js"
    },
    {
      "name": "Skeleton",
      "normalized": "skeleton",
      "source": "packages/mui-material/src/Skeleton",
      "declared_in": "packages/mui-material/src/Skeleton/index.js"
    },
    {
      "name": "Slide",
      "normalized": "slide",
      "source": "packages/mui-material/src/Slide",
      "declared_in": "packages/mui-material/src/Slide/index.js"
    },
    {
      "name": "Slider",
      "normalized": "slider",
      "source": "packages/mui-material/src/Slider",
      "declared_in": "packages/mui-material/src/Slider/index.js"
    },
    {
      "name": "SnackbarContent",
      "normalized": "snackbar_content",
      "source": "packages/mui-material/src/SnackbarContent",
      "declared_in": "packages/mui-material/src/SnackbarContent/index.js"
    },
    {
      "name": "SpeedDial",
      "normalized": "speed_dial",
      "source": "packages/mui-material/src/SpeedDial",
      "declared_in": "packages/mui-material/src/SpeedDial/index.js"
    },
    {
      "name": "SpeedDialAction",
      "normalized": "speed_dial_action",
      "source": "packages/mui-material/src/SpeedDialAction",
      "declared_in": "packages/mui-material/src/SpeedDialAction/index.js"
    },
    {
      "name": "SpeedDialIcon",
      "normalized": "speed_dial_icon",
      "source": "packages/mui-material/src/SpeedDialIcon",
      "declared_in": "packages/mui-material/src/SpeedDialIcon/index.js"
    },
    {
      "name": "Stack",
      "normalized": "stack",
      "source": "packages/mui-material/src/Stack",
      "declared_in": "packages/mui-material/src/Stack/index.js"
    },
    {
      "name": "Step",
      "normalized": "step",
      "source": "packages/mui-material/src/Step",
      "declared_in": "packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepButton",
      "normalized": "step_button",
      "source": "packages/mui-material/src/StepButton",
      "declared_in": "packages/mui-material/src/StepButton/index.js"
    },
    {
      "name": "StepConnector",
      "normalized": "step_connector",
      "source": "packages/mui-material/src/StepConnector",
      "declared_in": "packages/mui-material/src/StepConnector/index.js"
    },
    {
      "name": "StepContent",
      "normalized": "step_content",
      "source": "packages/mui-material/src/StepContent",
      "declared_in": "packages/mui-material/src/StepContent/index.js"
    },
    {
      "name": "StepContext",
      "normalized": "step_context",
      "source": "packages/mui-material/src/StepContext",
      "declared_in": "packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepIcon",
      "normalized": "step_icon",
      "source": "packages/mui-material/src/StepIcon",
      "declared_in": "packages/mui-material/src/StepIcon/index.js"
    },
    {
      "name": "StepLabel",
      "normalized": "step_label",
      "source": "packages/mui-material/src/StepLabel",
      "declared_in": "packages/mui-material/src/StepLabel/index.js"
    },
    {
      "name": "Stepper",
      "normalized": "stepper",
      "source": "packages/mui-material/src/Stepper",
      "declared_in": "packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "StepperContext",
      "normalized": "stepper_context",
      "source": "packages/mui-material/src/StepperContext",
      "declared_in": "packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "SvgIcon",
      "normalized": "svg_icon",
      "source": "packages/mui-material/src/SvgIcon",
      "declared_in": "packages/mui-material/src/SvgIcon/index.js"
    },
    {
      "name": "SwipeableDrawer",
      "normalized": "swipeable_drawer",
      "source": "packages/mui-material/src/SwipeableDrawer",
      "declared_in": "packages/mui-material/src/SwipeableDrawer/index.js"
    },
    {
      "name": "Switch",
      "normalized": "switch",
      "source": "packages/mui-material/src/Switch",
      "declared_in": "packages/mui-material/src/Switch/index.js"
    },
    {
      "name": "Tab",
      "normalized": "tab",
      "source": "packages/mui-material/src/Tab",
      "declared_in": "packages/mui-material/src/Tab/index.js"
    },
    {
      "name": "TabScrollButton",
      "normalized": "tab_scroll_button",
      "source": "packages/mui-material/src/TabScrollButton",
      "declared_in": "packages/mui-material/src/TabScrollButton/index.js"
    },
    {
      "name": "Table",
      "normalized": "table",
      "source": "packages/mui-material/src/Table",
      "declared_in": "packages/mui-material/src/Table/index.js"
    },
    {
      "name": "TableBody",
      "normalized": "table_body",
      "source": "packages/mui-material/src/TableBody",
      "declared_in": "packages/mui-material/src/TableBody/index.js"
    },
    {
      "name": "TableCell",
      "normalized": "table_cell",
      "source": "packages/mui-material/src/TableCell",
      "declared_in": "packages/mui-material/src/TableCell/index.js"
    },
    {
      "name": "TableContainer",
      "normalized": "table_container",
      "source": "packages/mui-material/src/TableContainer",
      "declared_in": "packages/mui-material/src/TableContainer/index.js"
    },
    {
      "name": "TableFooter",
      "normalized": "table_footer",
      "source": "packages/mui-material/src/TableFooter",
      "declared_in": "packages/mui-material/src/TableFooter/index.js"
    },
    {
      "name": "TableHead",
      "normalized": "table_head",
      "source": "packages/mui-material/src/TableHead",
      "declared_in": "packages/mui-material/src/TableHead/index.js"
    },
    {
      "name": "TablePagination",
      "normalized": "table_pagination",
      "source": "packages/mui-material/src/TablePagination",
      "declared_in": "packages/mui-material/src/TablePagination/index.js"
    },
    {
      "name": "TablePaginationActions",
      "normalized": "table_pagination_actions",
      "source": "packages/mui-material/src/TablePaginationActions",
      "declared_in": "packages/mui-material/src/TablePaginationActions/index.js"
    },
    {
      "name": "TableRow",
      "normalized": "table_row",
      "source": "packages/mui-material/src/TableRow",
      "declared_in": "packages/mui-material/src/TableRow/index.js"
    },
    {
      "name": "TableSortLabel",
      "normalized": "table_sort_label",
      "source": "packages/mui-material/src/TableSortLabel",
      "declared_in": "packages/mui-material/src/TableSortLabel/index.js"
    },
    {
      "name": "Tabs",
      "normalized": "tabs",
      "source": "packages/mui-material/src/Tabs",
      "declared_in": "packages/mui-material/src/Tabs/index.js"
    },
    {
      "name": "TextareaAutosize",
      "normalized": "textarea_autosize",
      "source": "packages/mui-material/src/TextareaAutosize",
      "declared_in": "packages/mui-material/src/TextareaAutosize/index.js"
    },
    {
      "name": "THEME_ID",
      "normalized": "theme_id",
      "source": "packages/mui-material/src/identifier",
      "declared_in": "packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ThemeProvider",
      "normalized": "theme_provider",
      "source": "packages/mui-material/src/ThemeProvider",
      "declared_in": "packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ToggleButton",
      "normalized": "toggle_button",
      "source": "packages/mui-material/src/ToggleButton",
      "declared_in": "packages/mui-material/src/ToggleButton/index.js"
    },
    {
      "name": "ToggleButtonGroup",
      "normalized": "toggle_button_group",
      "source": "packages/mui-material/src/ToggleButtonGroup",
      "declared_in": "packages/mui-material/src/ToggleButtonGroup/index.js"
    },
    {
      "name": "Toolbar",
      "normalized": "toolbar",
      "source": "packages/mui-material/src/Toolbar",
      "declared_in": "packages/mui-material/src/Toolbar/index.js"
    },
    {
      "name": "Tooltip",
      "normalized": "tooltip",
      "source": "packages/mui-material/src/Tooltip",
      "declared_in": "packages/mui-material/src/Tooltip/index.js"
    },
    {
      "name": "Typography",
      "normalized": "typography",
      "source": "packages/mui-material/src/Typography",
      "declared_in": "packages/mui-material/src/Typography/index.js"
    },
    {
      "name": "Unstable_TrapFocus",
      "normalized": "unstable_trap_focus",
      "source": "packages/mui-material/src/Unstable_TrapFocus",
      "declared_in": "packages/mui-material/src/index.js"
    },
    {
      "name": "UseAutocomplete",
      "normalized": "use_autocomplete",
      "source": "packages/mui-material/src/useAutocomplete",
      "declared_in": "packages/mui-material/src/useAutocomplete/index.js"
    },
    {
      "name": "UseLazyRipple",
      "normalized": "use_lazy_ripple",
      "source": "packages/mui-material/src/useLazyRipple",
      "declared_in": "packages/mui-material/src/useLazyRipple/index.ts"
    },
    {
      "name": "UsePagination",
      "normalized": "use_pagination",
      "source": "packages/mui-material/src/usePagination",
      "declared_in": "packages/mui-material/src/usePagination/index.js"
    },
    {
      "name": "UseScrollTrigger",
      "normalized": "use_scroll_trigger",
      "source": "packages/mui-material/src/useScrollTrigger",
      "declared_in": "packages/mui-material/src/useScrollTrigger/index.js"
    },
    {
      "name": "Zoom",
      "normalized": "zoom",
      "source": "packages/mui-material/src/Zoom",
      "declared_in": "packages/mui-material/src/Zoom/index.js"
    }
  ],
  "missing_from_headless": [
    {
      "name": "Accordion",
      "normalized": "accordion",
      "source": "packages/mui-material/src/Accordion",
      "declared_in": "packages/mui-material/src/Accordion/index.js"
    },
    {
      "name": "AccordionActions",
      "normalized": "accordion_actions",
      "source": "packages/mui-material/src/AccordionActions",
      "declared_in": "packages/mui-material/src/AccordionActions/index.js"
    },
    {
      "name": "AccordionDetails",
      "normalized": "accordion_details",
      "source": "packages/mui-material/src/AccordionDetails",
      "declared_in": "packages/mui-material/src/AccordionDetails/index.js"
    },
    {
      "name": "AccordionSummary",
      "normalized": "accordion_summary",
      "source": "packages/mui-material/src/AccordionSummary",
      "declared_in": "packages/mui-material/src/AccordionSummary/index.js"
    },
    {
      "name": "Alert",
      "normalized": "alert",
      "source": "packages/mui-material/src/Alert",
      "declared_in": "packages/mui-material/src/Alert/index.js"
    },
    {
      "name": "AlertTitle",
      "normalized": "alert_title",
      "source": "packages/mui-material/src/AlertTitle",
      "declared_in": "packages/mui-material/src/AlertTitle/index.js"
    },
    {
      "name": "AppBar",
      "normalized": "app_bar",
      "source": "packages/mui-material/src/AppBar",
      "declared_in": "packages/mui-material/src/AppBar/index.js"
    },
    {
      "name": "Autocomplete",
      "normalized": "autocomplete",
      "source": "packages/mui-material/src/Autocomplete",
      "declared_in": "packages/mui-material/src/Autocomplete/index.js"
    },
    {
      "name": "Avatar",
      "normalized": "avatar",
      "source": "packages/mui-material/src/Avatar",
      "declared_in": "packages/mui-material/src/Avatar/index.js"
    },
    {
      "name": "AvatarGroup",
      "normalized": "avatar_group",
      "source": "packages/mui-material/src/AvatarGroup",
      "declared_in": "packages/mui-material/src/AvatarGroup/index.js"
    },
    {
      "name": "Backdrop",
      "normalized": "backdrop",
      "source": "packages/mui-material/src/Backdrop",
      "declared_in": "packages/mui-material/src/Backdrop/index.js"
    },
    {
      "name": "Badge",
      "normalized": "badge",
      "source": "packages/mui-material/src/Badge",
      "declared_in": "packages/mui-material/src/Badge/index.js"
    },
    {
      "name": "BottomNavigation",
      "normalized": "bottom_navigation",
      "source": "packages/mui-material/src/BottomNavigation",
      "declared_in": "packages/mui-material/src/BottomNavigation/index.js"
    },
    {
      "name": "BottomNavigationAction",
      "normalized": "bottom_navigation_action",
      "source": "packages/mui-material/src/BottomNavigationAction",
      "declared_in": "packages/mui-material/src/BottomNavigationAction/index.js"
    },
    {
      "name": "Box",
      "normalized": "box",
      "source": "packages/mui-material/src/Box",
      "declared_in": "packages/mui-material/src/Box/index.js"
    },
    {
      "name": "Breadcrumbs",
      "normalized": "breadcrumbs",
      "source": "packages/mui-material/src/Breadcrumbs",
      "declared_in": "packages/mui-material/src/Breadcrumbs/index.js"
    },
    {
      "name": "ButtonBase",
      "normalized": "button_base",
      "source": "packages/mui-material/src/ButtonBase",
      "declared_in": "packages/mui-material/src/ButtonBase/index.js"
    },
    {
      "name": "ButtonGroup",
      "normalized": "button_group",
      "source": "packages/mui-material/src/ButtonGroup",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupButtonContext",
      "normalized": "button_group_button_context",
      "source": "packages/mui-material/src/ButtonGroupButtonContext",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupContext",
      "normalized": "button_group_context",
      "source": "packages/mui-material/src/ButtonGroupContext",
      "declared_in": "packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "Card",
      "normalized": "card",
      "source": "packages/mui-material/src/Card",
      "declared_in": "packages/mui-material/src/Card/index.js"
    },
    {
      "name": "CardActionArea",
      "normalized": "card_action_area",
      "source": "packages/mui-material/src/CardActionArea",
      "declared_in": "packages/mui-material/src/CardActionArea/index.js"
    },
    {
      "name": "CardActions",
      "normalized": "card_actions",
      "source": "packages/mui-material/src/CardActions",
      "declared_in": "packages/mui-material/src/CardActions/index.js"
    },
    {
      "name": "CardContent",
      "normalized": "card_content",
      "source": "packages/mui-material/src/CardContent",
      "declared_in": "packages/mui-material/src/CardContent/index.js"
    },
    {
      "name": "CardHeader",
      "normalized": "card_header",
      "source": "packages/mui-material/src/CardHeader",
      "declared_in": "packages/mui-material/src/CardHeader/index.js"
    },
    {
      "name": "CardMedia",
      "normalized": "card_media",
      "source": "packages/mui-material/src/CardMedia",
      "declared_in": "packages/mui-material/src/CardMedia/index.js"
    },
    {
      "name": "Checkbox",
      "normalized": "checkbox",
      "source": "packages/mui-material/src/Checkbox",
      "declared_in": "packages/mui-material/src/Checkbox/index.js"
    },
    {
      "name": "Chip",
      "normalized": "chip",
      "source": "packages/mui-material/src/Chip",
      "declared_in": "packages/mui-material/src/Chip/index.js"
    },
    {
      "name": "CircularProgress",
      "normalized": "circular_progress",
      "source": "packages/mui-material/src/CircularProgress",
      "declared_in": "packages/mui-material/src/CircularProgress/index.js"
    },
    {
      "name": "ClickAwayListener",
      "normalized": "click_away_listener",
      "source": "packages/mui-material/src/ClickAwayListener",
      "declared_in": "packages/mui-material/src/index.js"
    },
    {
      "name": "Collapse",
      "normalized": "collapse",
      "source": "packages/mui-material/src/Collapse",
      "declared_in": "packages/mui-material/src/Collapse/index.js"
    },
    {
      "name": "Container",
      "normalized": "container",
      "source": "packages/mui-material/src/Container",
      "declared_in": "packages/mui-material/src/Container/index.js"
    },
    {
      "name": "CssBaseline",
      "normalized": "css_baseline",
      "source": "packages/mui-material/src/CssBaseline",
      "declared_in": "packages/mui-material/src/CssBaseline/index.js"
    },
    {
      "name": "DefaultPropsProvider",
      "normalized": "default_props_provider",
      "source": "packages/mui-material/src/DefaultPropsProvider",
      "declared_in": "packages/mui-material/src/DefaultPropsProvider/index.ts"
    },
    {
      "name": "Dialog",
      "normalized": "dialog",
      "source": "packages/mui-material/src/Dialog",
      "declared_in": "packages/mui-material/src/Dialog/index.js"
    },
    {
      "name": "DialogActions",
      "normalized": "dialog_actions",
      "source": "packages/mui-material/src/DialogActions",
      "declared_in": "packages/mui-material/src/DialogActions/index.js"
    },
    {
      "name": "DialogContent",
      "normalized": "dialog_content",
      "source": "packages/mui-material/src/DialogContent",
      "declared_in": "packages/mui-material/src/DialogContent/index.js"
    },
    {
      "name": "DialogContentText",
      "normalized": "dialog_content_text",
      "source": "packages/mui-material/src/DialogContentText",
      "declared_in": "packages/mui-material/src/DialogContentText/index.js"
    },
    {
      "name": "DialogTitle",
      "normalized": "dialog_title",
      "source": "packages/mui-material/src/DialogTitle",
      "declared_in": "packages/mui-material/src/DialogTitle/index.js"
    },
    {
      "name": "Divider",
      "normalized": "divider",
      "source": "packages/mui-material/src/Divider",
      "declared_in": "packages/mui-material/src/Divider/index.js"
    },
    {
      "name": "Drawer",
      "normalized": "drawer",
      "source": "packages/mui-material/src/Drawer",
      "declared_in": "packages/mui-material/src/Drawer/index.js"
    },
    {
      "name": "Fab",
      "normalized": "fab",
      "source": "packages/mui-material/src/Fab",
      "declared_in": "packages/mui-material/src/Fab/index.js"
    },
    {
      "name": "Fade",
      "normalized": "fade",
      "source": "packages/mui-material/src/Fade",
      "declared_in": "packages/mui-material/src/Fade/index.js"
    },
    {
      "name": "FilledInput",
      "normalized": "filled_input",
      "source": "packages/mui-material/src/FilledInput",
      "declared_in": "packages/mui-material/src/FilledInput/index.js"
    },
    {
      "name": "FocusTrap",
      "normalized": "focus_trap",
      "source": "packages/mui-material/src/FocusTrap",
      "declared_in": "packages/mui-material/src/Unstable_TrapFocus/index.js"
    },
    {
      "name": "FormControl",
      "normalized": "form_control",
      "source": "packages/mui-material/src/FormControl",
      "declared_in": "packages/mui-material/src/FormControl/index.js"
    },
    {
      "name": "FormControlLabel",
      "normalized": "form_control_label",
      "source": "packages/mui-material/src/FormControlLabel",
      "declared_in": "packages/mui-material/src/FormControlLabel/index.js"
    },
    {
      "name": "FormGroup",
      "normalized": "form_group",
      "source": "packages/mui-material/src/FormGroup",
      "declared_in": "packages/mui-material/src/FormGroup/index.js"
    },
    {
      "name": "FormHelperText",
      "normalized": "form_helper_text",
      "source": "packages/mui-material/src/FormHelperText",
      "declared_in": "packages/mui-material/src/FormHelperText/index.js"
    },
    {
      "name": "FormLabel",
      "normalized": "form_label",
      "source": "packages/mui-material/src/FormLabel",
      "declared_in": "packages/mui-material/src/FormLabel/index.js"
    },
    {
      "name": "GlobalStyles",
      "normalized": "global_styles",
      "source": "packages/mui-material/src/GlobalStyles",
      "declared_in": "packages/mui-material/src/GlobalStyles/index.js"
    },
    {
      "name": "Grid",
      "normalized": "grid",
      "source": "packages/mui-material/src/Grid",
      "declared_in": "packages/mui-material/src/Grid/index.ts"
    },
    {
      "name": "GridLegacy",
      "normalized": "grid_legacy",
      "source": "packages/mui-material/src/GridLegacy",
      "declared_in": "packages/mui-material/src/GridLegacy/index.js"
    },
    {
      "name": "Grow",
      "normalized": "grow",
      "source": "packages/mui-material/src/Grow",
      "declared_in": "packages/mui-material/src/Grow/index.js"
    },
    {
      "name": "Icon",
      "normalized": "icon",
      "source": "packages/mui-material/src/Icon",
      "declared_in": "packages/mui-material/src/Icon/index.js"
    },
    {
      "name": "IconButton",
      "normalized": "icon_button",
      "source": "packages/mui-material/src/IconButton",
      "declared_in": "packages/mui-material/src/IconButton/index.js"
    },
    {
      "name": "ImageList",
      "normalized": "image_list",
      "source": "packages/mui-material/src/ImageList",
      "declared_in": "packages/mui-material/src/ImageList/index.js"
    },
    {
      "name": "ImageListItem",
      "normalized": "image_list_item",
      "source": "packages/mui-material/src/ImageListItem",
      "declared_in": "packages/mui-material/src/ImageListItem/index.js"
    },
    {
      "name": "ImageListItemBar",
      "normalized": "image_list_item_bar",
      "source": "packages/mui-material/src/ImageListItemBar",
      "declared_in": "packages/mui-material/src/ImageListItemBar/index.js"
    },
    {
      "name": "InitColorSchemeScript",
      "normalized": "init_color_scheme_script",
      "source": "packages/mui-material/src/InitColorSchemeScript",
      "declared_in": "packages/mui-material/src/InitColorSchemeScript/index.ts"
    },
    {
      "name": "Input",
      "normalized": "input",
      "source": "packages/mui-material/src/Input",
      "declared_in": "packages/mui-material/src/Input/index.js"
    },
    {
      "name": "InputAdornment",
      "normalized": "input_adornment",
      "source": "packages/mui-material/src/InputAdornment",
      "declared_in": "packages/mui-material/src/InputAdornment/index.js"
    },
    {
      "name": "InputBase",
      "normalized": "input_base",
      "source": "packages/mui-material/src/InputBase",
      "declared_in": "packages/mui-material/src/InputBase/index.js"
    },
    {
      "name": "InputLabel",
      "normalized": "input_label",
      "source": "packages/mui-material/src/InputLabel",
      "declared_in": "packages/mui-material/src/InputLabel/index.js"
    },
    {
      "name": "LinearProgress",
      "normalized": "linear_progress",
      "source": "packages/mui-material/src/LinearProgress",
      "declared_in": "packages/mui-material/src/LinearProgress/index.js"
    },
    {
      "name": "Link",
      "normalized": "link",
      "source": "packages/mui-material/src/Link",
      "declared_in": "packages/mui-material/src/Link/index.js"
    },
    {
      "name": "List",
      "normalized": "list",
      "source": "packages/mui-material/src/List",
      "declared_in": "packages/mui-material/src/List/index.js"
    },
    {
      "name": "ListItem",
      "normalized": "list_item",
      "source": "packages/mui-material/src/ListItem",
      "declared_in": "packages/mui-material/src/ListItem/index.js"
    },
    {
      "name": "ListItemAvatar",
      "normalized": "list_item_avatar",
      "source": "packages/mui-material/src/ListItemAvatar",
      "declared_in": "packages/mui-material/src/ListItemAvatar/index.js"
    },
    {
      "name": "ListItemButton",
      "normalized": "list_item_button",
      "source": "packages/mui-material/src/ListItemButton",
      "declared_in": "packages/mui-material/src/ListItemButton/index.js"
    },
    {
      "name": "ListItemIcon",
      "normalized": "list_item_icon",
      "source": "packages/mui-material/src/ListItemIcon",
      "declared_in": "packages/mui-material/src/ListItemIcon/index.js"
    },
    {
      "name": "ListItemSecondaryAction",
      "normalized": "list_item_secondary_action",
      "source": "packages/mui-material/src/ListItemSecondaryAction",
      "declared_in": "packages/mui-material/src/ListItemSecondaryAction/index.js"
    },
    {
      "name": "ListItemText",
      "normalized": "list_item_text",
      "source": "packages/mui-material/src/ListItemText",
      "declared_in": "packages/mui-material/src/ListItemText/index.js"
    },
    {
      "name": "ListSubheader",
      "normalized": "list_subheader",
      "source": "packages/mui-material/src/ListSubheader",
      "declared_in": "packages/mui-material/src/ListSubheader/index.js"
    },
    {
      "name": "Menu",
      "normalized": "menu",
      "source": "packages/mui-material/src/Menu",
      "declared_in": "packages/mui-material/src/Menu/index.js"
    },
    {
      "name": "MenuItem",
      "normalized": "menu_item",
      "source": "packages/mui-material/src/MenuItem",
      "declared_in": "packages/mui-material/src/MenuItem/index.js"
    },
    {
      "name": "MenuList",
      "normalized": "menu_list",
      "source": "packages/mui-material/src/MenuList",
      "declared_in": "packages/mui-material/src/MenuList/index.js"
    },
    {
      "name": "MobileStepper",
      "normalized": "mobile_stepper",
      "source": "packages/mui-material/src/MobileStepper",
      "declared_in": "packages/mui-material/src/MobileStepper/index.js"
    },
    {
      "name": "Modal",
      "normalized": "modal",
      "source": "packages/mui-material/src/Modal",
      "declared_in": "packages/mui-material/src/Modal/index.js"
    },
    {
      "name": "NativeSelect",
      "normalized": "native_select",
      "source": "packages/mui-material/src/NativeSelect",
      "declared_in": "packages/mui-material/src/NativeSelect/index.js"
    },
    {
      "name": "NoSsr",
      "normalized": "no_ssr",
      "source": "packages/mui-material/src/NoSsr",
      "declared_in": "packages/mui-material/src/NoSsr/index.js"
    },
    {
      "name": "OutlinedInput",
      "normalized": "outlined_input",
      "source": "packages/mui-material/src/OutlinedInput",
      "declared_in": "packages/mui-material/src/OutlinedInput/index.js"
    },
    {
      "name": "Pagination",
      "normalized": "pagination",
      "source": "packages/mui-material/src/Pagination",
      "declared_in": "packages/mui-material/src/Pagination/index.js"
    },
    {
      "name": "PaginationItem",
      "normalized": "pagination_item",
      "source": "packages/mui-material/src/PaginationItem",
      "declared_in": "packages/mui-material/src/PaginationItem/index.js"
    },
    {
      "name": "Paper",
      "normalized": "paper",
      "source": "packages/mui-material/src/Paper",
      "declared_in": "packages/mui-material/src/Paper/index.js"
    },
    {
      "name": "PigmentContainer",
      "normalized": "pigment_container",
      "source": "packages/mui-material/src/PigmentContainer",
      "declared_in": "packages/mui-material/src/PigmentContainer/index.ts"
    },
    {
      "name": "PigmentGrid",
      "normalized": "pigment_grid",
      "source": "packages/mui-material/src/PigmentGrid",
      "declared_in": "packages/mui-material/src/PigmentGrid/index.ts"
    },
    {
      "name": "PigmentStack",
      "normalized": "pigment_stack",
      "source": "packages/mui-material/src/PigmentStack",
      "declared_in": "packages/mui-material/src/PigmentStack/index.ts"
    },
    {
      "name": "Popover",
      "normalized": "popover",
      "source": "packages/mui-material/src/Popover",
      "declared_in": "packages/mui-material/src/Popover/index.js"
    },
    {
      "name": "Popper",
      "normalized": "popper",
      "source": "packages/mui-material/src/Popper",
      "declared_in": "packages/mui-material/src/Popper/index.js"
    },
    {
      "name": "Portal",
      "normalized": "portal",
      "source": "packages/mui-material/src/Portal",
      "declared_in": "packages/mui-material/src/Portal/index.js"
    },
    {
      "name": "Radio",
      "normalized": "radio",
      "source": "packages/mui-material/src/Radio",
      "declared_in": "packages/mui-material/src/Radio/index.js"
    },
    {
      "name": "RadioGroup",
      "normalized": "radio_group",
      "source": "packages/mui-material/src/RadioGroup",
      "declared_in": "packages/mui-material/src/RadioGroup/index.js"
    },
    {
      "name": "Rating",
      "normalized": "rating",
      "source": "packages/mui-material/src/Rating",
      "declared_in": "packages/mui-material/src/Rating/index.js"
    },
    {
      "name": "ScopedCssBaseline",
      "normalized": "scoped_css_baseline",
      "source": "packages/mui-material/src/ScopedCssBaseline",
      "declared_in": "packages/mui-material/src/ScopedCssBaseline/index.js"
    },
    {
      "name": "Select",
      "normalized": "select",
      "source": "packages/mui-material/src/Select",
      "declared_in": "packages/mui-material/src/Select/index.js"
    },
    {
      "name": "Skeleton",
      "normalized": "skeleton",
      "source": "packages/mui-material/src/Skeleton",
      "declared_in": "packages/mui-material/src/Skeleton/index.js"
    },
    {
      "name": "Slide",
      "normalized": "slide",
      "source": "packages/mui-material/src/Slide",
      "declared_in": "packages/mui-material/src/Slide/index.js"
    },
    {
      "name": "Slider",
      "normalized": "slider",
      "source": "packages/mui-material/src/Slider",
      "declared_in": "packages/mui-material/src/Slider/index.js"
    },
    {
      "name": "Snackbar",
      "normalized": "snackbar",
      "source": "packages/mui-material/src/Snackbar",
      "declared_in": "packages/mui-material/src/Snackbar/index.js"
    },
    {
      "name": "SnackbarContent",
      "normalized": "snackbar_content",
      "source": "packages/mui-material/src/SnackbarContent",
      "declared_in": "packages/mui-material/src/SnackbarContent/index.js"
    },
    {
      "name": "SpeedDial",
      "normalized": "speed_dial",
      "source": "packages/mui-material/src/SpeedDial",
      "declared_in": "packages/mui-material/src/SpeedDial/index.js"
    },
    {
      "name": "SpeedDialAction",
      "normalized": "speed_dial_action",
      "source": "packages/mui-material/src/SpeedDialAction",
      "declared_in": "packages/mui-material/src/SpeedDialAction/index.js"
    },
    {
      "name": "SpeedDialIcon",
      "normalized": "speed_dial_icon",
      "source": "packages/mui-material/src/SpeedDialIcon",
      "declared_in": "packages/mui-material/src/SpeedDialIcon/index.js"
    },
    {
      "name": "Stack",
      "normalized": "stack",
      "source": "packages/mui-material/src/Stack",
      "declared_in": "packages/mui-material/src/Stack/index.js"
    },
    {
      "name": "Step",
      "normalized": "step",
      "source": "packages/mui-material/src/Step",
      "declared_in": "packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepButton",
      "normalized": "step_button",
      "source": "packages/mui-material/src/StepButton",
      "declared_in": "packages/mui-material/src/StepButton/index.js"
    },
    {
      "name": "StepConnector",
      "normalized": "step_connector",
      "source": "packages/mui-material/src/StepConnector",
      "declared_in": "packages/mui-material/src/StepConnector/index.js"
    },
    {
      "name": "StepContent",
      "normalized": "step_content",
      "source": "packages/mui-material/src/StepContent",
      "declared_in": "packages/mui-material/src/StepContent/index.js"
    },
    {
      "name": "StepContext",
      "normalized": "step_context",
      "source": "packages/mui-material/src/StepContext",
      "declared_in": "packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepIcon",
      "normalized": "step_icon",
      "source": "packages/mui-material/src/StepIcon",
      "declared_in": "packages/mui-material/src/StepIcon/index.js"
    },
    {
      "name": "StepLabel",
      "normalized": "step_label",
      "source": "packages/mui-material/src/StepLabel",
      "declared_in": "packages/mui-material/src/StepLabel/index.js"
    },
    {
      "name": "Stepper",
      "normalized": "stepper",
      "source": "packages/mui-material/src/Stepper",
      "declared_in": "packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "StepperContext",
      "normalized": "stepper_context",
      "source": "packages/mui-material/src/StepperContext",
      "declared_in": "packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "SvgIcon",
      "normalized": "svg_icon",
      "source": "packages/mui-material/src/SvgIcon",
      "declared_in": "packages/mui-material/src/SvgIcon/index.js"
    },
    {
      "name": "SwipeableDrawer",
      "normalized": "swipeable_drawer",
      "source": "packages/mui-material/src/SwipeableDrawer",
      "declared_in": "packages/mui-material/src/SwipeableDrawer/index.js"
    },
    {
      "name": "Switch",
      "normalized": "switch",
      "source": "packages/mui-material/src/Switch",
      "declared_in": "packages/mui-material/src/Switch/index.js"
    },
    {
      "name": "Tab",
      "normalized": "tab",
      "source": "packages/mui-material/src/Tab",
      "declared_in": "packages/mui-material/src/Tab/index.js"
    },
    {
      "name": "TabScrollButton",
      "normalized": "tab_scroll_button",
      "source": "packages/mui-material/src/TabScrollButton",
      "declared_in": "packages/mui-material/src/TabScrollButton/index.js"
    },
    {
      "name": "Table",
      "normalized": "table",
      "source": "packages/mui-material/src/Table",
      "declared_in": "packages/mui-material/src/Table/index.js"
    },
    {
      "name": "TableBody",
      "normalized": "table_body",
      "source": "packages/mui-material/src/TableBody",
      "declared_in": "packages/mui-material/src/TableBody/index.js"
    },
    {
      "name": "TableCell",
      "normalized": "table_cell",
      "source": "packages/mui-material/src/TableCell",
      "declared_in": "packages/mui-material/src/TableCell/index.js"
    },
    {
      "name": "TableContainer",
      "normalized": "table_container",
      "source": "packages/mui-material/src/TableContainer",
      "declared_in": "packages/mui-material/src/TableContainer/index.js"
    },
    {
      "name": "TableFooter",
      "normalized": "table_footer",
      "source": "packages/mui-material/src/TableFooter",
      "declared_in": "packages/mui-material/src/TableFooter/index.js"
    },
    {
      "name": "TableHead",
      "normalized": "table_head",
      "source": "packages/mui-material/src/TableHead",
      "declared_in": "packages/mui-material/src/TableHead/index.js"
    },
    {
      "name": "TablePagination",
      "normalized": "table_pagination",
      "source": "packages/mui-material/src/TablePagination",
      "declared_in": "packages/mui-material/src/TablePagination/index.js"
    },
    {
      "name": "TablePaginationActions",
      "normalized": "table_pagination_actions",
      "source": "packages/mui-material/src/TablePaginationActions",
      "declared_in": "packages/mui-material/src/TablePaginationActions/index.js"
    },
    {
      "name": "TableRow",
      "normalized": "table_row",
      "source": "packages/mui-material/src/TableRow",
      "declared_in": "packages/mui-material/src/TableRow/index.js"
    },
    {
      "name": "TableSortLabel",
      "normalized": "table_sort_label",
      "source": "packages/mui-material/src/TableSortLabel",
      "declared_in": "packages/mui-material/src/TableSortLabel/index.js"
    },
    {
      "name": "Tabs",
      "normalized": "tabs",
      "source": "packages/mui-material/src/Tabs",
      "declared_in": "packages/mui-material/src/Tabs/index.js"
    },
    {
      "name": "TextField",
      "normalized": "text_field",
      "source": "packages/mui-material/src/TextField",
      "declared_in": "packages/mui-material/src/TextField/index.js"
    },
    {
      "name": "TextareaAutosize",
      "normalized": "textarea_autosize",
      "source": "packages/mui-material/src/TextareaAutosize",
      "declared_in": "packages/mui-material/src/TextareaAutosize/index.js"
    },
    {
      "name": "THEME_ID",
      "normalized": "theme_id",
      "source": "packages/mui-material/src/identifier",
      "declared_in": "packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ThemeProvider",
      "normalized": "theme_provider",
      "source": "packages/mui-material/src/ThemeProvider",
      "declared_in": "packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ToggleButton",
      "normalized": "toggle_button",
      "source": "packages/mui-material/src/ToggleButton",
      "declared_in": "packages/mui-material/src/ToggleButton/index.js"
    },
    {
      "name": "ToggleButtonGroup",
      "normalized": "toggle_button_group",
      "source": "packages/mui-material/src/ToggleButtonGroup",
      "declared_in": "packages/mui-material/src/ToggleButtonGroup/index.js"
    },
    {
      "name": "Toolbar",
      "normalized": "toolbar",
      "source": "packages/mui-material/src/Toolbar",
      "declared_in": "packages/mui-material/src/Toolbar/index.js"
    },
    {
      "name": "Tooltip",
      "normalized": "tooltip",
      "source": "packages/mui-material/src/Tooltip",
      "declared_in": "packages/mui-material/src/Tooltip/index.js"
    },
    {
      "name": "Typography",
      "normalized": "typography",
      "source": "packages/mui-material/src/Typography",
      "declared_in": "packages/mui-material/src/Typography/index.js"
    },
    {
      "name": "Unstable_TrapFocus",
      "normalized": "unstable_trap_focus",
      "source": "packages/mui-material/src/Unstable_TrapFocus",
      "declared_in": "packages/mui-material/src/index.js"
    },
    {
      "name": "UseAutocomplete",
      "normalized": "use_autocomplete",
      "source": "packages/mui-material/src/useAutocomplete",
      "declared_in": "packages/mui-material/src/useAutocomplete/index.js"
    },
    {
      "name": "UseLazyRipple",
      "normalized": "use_lazy_ripple",
      "source": "packages/mui-material/src/useLazyRipple",
      "declared_in": "packages/mui-material/src/useLazyRipple/index.ts"
    },
    {
      "name": "UsePagination",
      "normalized": "use_pagination",
      "source": "packages/mui-material/src/usePagination",
      "declared_in": "packages/mui-material/src/usePagination/index.js"
    },
    {
      "name": "UseScrollTrigger",
      "normalized": "use_scroll_trigger",
      "source": "packages/mui-material/src/useScrollTrigger",
      "declared_in": "packages/mui-material/src/useScrollTrigger/index.js"
    },
    {
      "name": "Zoom",
      "normalized": "zoom",
      "source": "packages/mui-material/src/Zoom",
      "declared_in": "packages/mui-material/src/Zoom/index.js"
    }
  ],
  "extra_in_material": [],
  "extra_in_headless": [
    "aria"
  ]
}
```
