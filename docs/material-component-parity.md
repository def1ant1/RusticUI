# Material Component Parity

_Last updated 2025-09-19T13:05:32.393450968+00:00 via `cargo xtask material-parity`._

> **Archive mapping:** Legacy Material UI references now resolve from `archives/mui-packages/`.
> Enterprise adopters can trace the frozen JavaScript sources via these paths without touching the
> Rust-first crates that power new development.

## Coverage snapshot

- React exports analyzed: 146\n- `mui-material` coverage: 6 (4.1%)\n- `mui-headless` coverage: 1 (0.7%)\n
## Highest priority gaps

| Rank | Component | Source |
| --- | --- | --- |
| 1 | Accordion | `archives/mui-packages/mui-material/src/Accordion` |
| 2 | AccordionActions | `archives/mui-packages/mui-material/src/AccordionActions` |
| 3 | AccordionDetails | `archives/mui-packages/mui-material/src/AccordionDetails` |
| 4 | AccordionSummary | `archives/mui-packages/mui-material/src/AccordionSummary` |
| 5 | Alert | `archives/mui-packages/mui-material/src/Alert` |
| 6 | AlertTitle | `archives/mui-packages/mui-material/src/AlertTitle` |
| 7 | Autocomplete | `archives/mui-packages/mui-material/src/Autocomplete` |
| 8 | Avatar | `archives/mui-packages/mui-material/src/Avatar` |
| 9 | AvatarGroup | `archives/mui-packages/mui-material/src/AvatarGroup` |
| 10 | Backdrop | `archives/mui-packages/mui-material/src/Backdrop` |

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
      "source": "archives/mui-packages/mui-material/src/Accordion",
      "declared_in": "archives/mui-packages/mui-material/src/Accordion/index.js"
    },
    {
      "name": "AccordionActions",
      "normalized": "accordion_actions",
      "source": "archives/mui-packages/mui-material/src/AccordionActions",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionActions/index.js"
    },
    {
      "name": "AccordionDetails",
      "normalized": "accordion_details",
      "source": "archives/mui-packages/mui-material/src/AccordionDetails",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionDetails/index.js"
    },
    {
      "name": "AccordionSummary",
      "normalized": "accordion_summary",
      "source": "archives/mui-packages/mui-material/src/AccordionSummary",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionSummary/index.js"
    },
    {
      "name": "Alert",
      "normalized": "alert",
      "source": "archives/mui-packages/mui-material/src/Alert",
      "declared_in": "archives/mui-packages/mui-material/src/Alert/index.js"
    },
    {
      "name": "AlertTitle",
      "normalized": "alert_title",
      "source": "archives/mui-packages/mui-material/src/AlertTitle",
      "declared_in": "archives/mui-packages/mui-material/src/AlertTitle/index.js"
    },
    {
      "name": "AppBar",
      "normalized": "app_bar",
      "source": "archives/mui-packages/mui-material/src/AppBar",
      "declared_in": "archives/mui-packages/mui-material/src/AppBar/index.js"
    },
    {
      "name": "Autocomplete",
      "normalized": "autocomplete",
      "source": "archives/mui-packages/mui-material/src/Autocomplete",
      "declared_in": "archives/mui-packages/mui-material/src/Autocomplete/index.js"
    },
    {
      "name": "Avatar",
      "normalized": "avatar",
      "source": "archives/mui-packages/mui-material/src/Avatar",
      "declared_in": "archives/mui-packages/mui-material/src/Avatar/index.js"
    },
    {
      "name": "AvatarGroup",
      "normalized": "avatar_group",
      "source": "archives/mui-packages/mui-material/src/AvatarGroup",
      "declared_in": "archives/mui-packages/mui-material/src/AvatarGroup/index.js"
    },
    {
      "name": "Backdrop",
      "normalized": "backdrop",
      "source": "archives/mui-packages/mui-material/src/Backdrop",
      "declared_in": "archives/mui-packages/mui-material/src/Backdrop/index.js"
    },
    {
      "name": "Badge",
      "normalized": "badge",
      "source": "archives/mui-packages/mui-material/src/Badge",
      "declared_in": "archives/mui-packages/mui-material/src/Badge/index.js"
    },
    {
      "name": "BottomNavigation",
      "normalized": "bottom_navigation",
      "source": "archives/mui-packages/mui-material/src/BottomNavigation",
      "declared_in": "archives/mui-packages/mui-material/src/BottomNavigation/index.js"
    },
    {
      "name": "BottomNavigationAction",
      "normalized": "bottom_navigation_action",
      "source": "archives/mui-packages/mui-material/src/BottomNavigationAction",
      "declared_in": "archives/mui-packages/mui-material/src/BottomNavigationAction/index.js"
    },
    {
      "name": "Box",
      "normalized": "box",
      "source": "archives/mui-packages/mui-material/src/Box",
      "declared_in": "archives/mui-packages/mui-material/src/Box/index.js"
    },
    {
      "name": "Breadcrumbs",
      "normalized": "breadcrumbs",
      "source": "archives/mui-packages/mui-material/src/Breadcrumbs",
      "declared_in": "archives/mui-packages/mui-material/src/Breadcrumbs/index.js"
    },
    {
      "name": "Button",
      "normalized": "button",
      "source": "archives/mui-packages/mui-material/src/Button",
      "declared_in": "archives/mui-packages/mui-material/src/Button/index.js"
    },
    {
      "name": "ButtonBase",
      "normalized": "button_base",
      "source": "archives/mui-packages/mui-material/src/ButtonBase",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonBase/index.js"
    },
    {
      "name": "ButtonGroup",
      "normalized": "button_group",
      "source": "archives/mui-packages/mui-material/src/ButtonGroup",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupButtonContext",
      "normalized": "button_group_button_context",
      "source": "archives/mui-packages/mui-material/src/ButtonGroupButtonContext",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupContext",
      "normalized": "button_group_context",
      "source": "archives/mui-packages/mui-material/src/ButtonGroupContext",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "Card",
      "normalized": "card",
      "source": "archives/mui-packages/mui-material/src/Card",
      "declared_in": "archives/mui-packages/mui-material/src/Card/index.js"
    },
    {
      "name": "CardActionArea",
      "normalized": "card_action_area",
      "source": "archives/mui-packages/mui-material/src/CardActionArea",
      "declared_in": "archives/mui-packages/mui-material/src/CardActionArea/index.js"
    },
    {
      "name": "CardActions",
      "normalized": "card_actions",
      "source": "archives/mui-packages/mui-material/src/CardActions",
      "declared_in": "archives/mui-packages/mui-material/src/CardActions/index.js"
    },
    {
      "name": "CardContent",
      "normalized": "card_content",
      "source": "archives/mui-packages/mui-material/src/CardContent",
      "declared_in": "archives/mui-packages/mui-material/src/CardContent/index.js"
    },
    {
      "name": "CardHeader",
      "normalized": "card_header",
      "source": "archives/mui-packages/mui-material/src/CardHeader",
      "declared_in": "archives/mui-packages/mui-material/src/CardHeader/index.js"
    },
    {
      "name": "CardMedia",
      "normalized": "card_media",
      "source": "archives/mui-packages/mui-material/src/CardMedia",
      "declared_in": "archives/mui-packages/mui-material/src/CardMedia/index.js"
    },
    {
      "name": "Checkbox",
      "normalized": "checkbox",
      "source": "archives/mui-packages/mui-material/src/Checkbox",
      "declared_in": "archives/mui-packages/mui-material/src/Checkbox/index.js"
    },
    {
      "name": "Chip",
      "normalized": "chip",
      "source": "archives/mui-packages/mui-material/src/Chip",
      "declared_in": "archives/mui-packages/mui-material/src/Chip/index.js"
    },
    {
      "name": "CircularProgress",
      "normalized": "circular_progress",
      "source": "archives/mui-packages/mui-material/src/CircularProgress",
      "declared_in": "archives/mui-packages/mui-material/src/CircularProgress/index.js"
    },
    {
      "name": "ClickAwayListener",
      "normalized": "click_away_listener",
      "source": "archives/mui-packages/mui-material/src/ClickAwayListener",
      "declared_in": "archives/mui-packages/mui-material/src/index.js"
    },
    {
      "name": "Collapse",
      "normalized": "collapse",
      "source": "archives/mui-packages/mui-material/src/Collapse",
      "declared_in": "archives/mui-packages/mui-material/src/Collapse/index.js"
    },
    {
      "name": "Container",
      "normalized": "container",
      "source": "archives/mui-packages/mui-material/src/Container",
      "declared_in": "archives/mui-packages/mui-material/src/Container/index.js"
    },
    {
      "name": "CssBaseline",
      "normalized": "css_baseline",
      "source": "archives/mui-packages/mui-material/src/CssBaseline",
      "declared_in": "archives/mui-packages/mui-material/src/CssBaseline/index.js"
    },
    {
      "name": "DefaultPropsProvider",
      "normalized": "default_props_provider",
      "source": "archives/mui-packages/mui-material/src/DefaultPropsProvider",
      "declared_in": "archives/mui-packages/mui-material/src/DefaultPropsProvider/index.ts"
    },
    {
      "name": "Dialog",
      "normalized": "dialog",
      "source": "archives/mui-packages/mui-material/src/Dialog",
      "declared_in": "archives/mui-packages/mui-material/src/Dialog/index.js"
    },
    {
      "name": "DialogActions",
      "normalized": "dialog_actions",
      "source": "archives/mui-packages/mui-material/src/DialogActions",
      "declared_in": "archives/mui-packages/mui-material/src/DialogActions/index.js"
    },
    {
      "name": "DialogContent",
      "normalized": "dialog_content",
      "source": "archives/mui-packages/mui-material/src/DialogContent",
      "declared_in": "archives/mui-packages/mui-material/src/DialogContent/index.js"
    },
    {
      "name": "DialogContentText",
      "normalized": "dialog_content_text",
      "source": "archives/mui-packages/mui-material/src/DialogContentText",
      "declared_in": "archives/mui-packages/mui-material/src/DialogContentText/index.js"
    },
    {
      "name": "DialogTitle",
      "normalized": "dialog_title",
      "source": "archives/mui-packages/mui-material/src/DialogTitle",
      "declared_in": "archives/mui-packages/mui-material/src/DialogTitle/index.js"
    },
    {
      "name": "Divider",
      "normalized": "divider",
      "source": "archives/mui-packages/mui-material/src/Divider",
      "declared_in": "archives/mui-packages/mui-material/src/Divider/index.js"
    },
    {
      "name": "Drawer",
      "normalized": "drawer",
      "source": "archives/mui-packages/mui-material/src/Drawer",
      "declared_in": "archives/mui-packages/mui-material/src/Drawer/index.js"
    },
    {
      "name": "Fab",
      "normalized": "fab",
      "source": "archives/mui-packages/mui-material/src/Fab",
      "declared_in": "archives/mui-packages/mui-material/src/Fab/index.js"
    },
    {
      "name": "Fade",
      "normalized": "fade",
      "source": "archives/mui-packages/mui-material/src/Fade",
      "declared_in": "archives/mui-packages/mui-material/src/Fade/index.js"
    },
    {
      "name": "FilledInput",
      "normalized": "filled_input",
      "source": "archives/mui-packages/mui-material/src/FilledInput",
      "declared_in": "archives/mui-packages/mui-material/src/FilledInput/index.js"
    },
    {
      "name": "FocusTrap",
      "normalized": "focus_trap",
      "source": "archives/mui-packages/mui-material/src/FocusTrap",
      "declared_in": "archives/mui-packages/mui-material/src/Unstable_TrapFocus/index.js"
    },
    {
      "name": "FormControl",
      "normalized": "form_control",
      "source": "archives/mui-packages/mui-material/src/FormControl",
      "declared_in": "archives/mui-packages/mui-material/src/FormControl/index.js"
    },
    {
      "name": "FormControlLabel",
      "normalized": "form_control_label",
      "source": "archives/mui-packages/mui-material/src/FormControlLabel",
      "declared_in": "archives/mui-packages/mui-material/src/FormControlLabel/index.js"
    },
    {
      "name": "FormGroup",
      "normalized": "form_group",
      "source": "archives/mui-packages/mui-material/src/FormGroup",
      "declared_in": "archives/mui-packages/mui-material/src/FormGroup/index.js"
    },
    {
      "name": "FormHelperText",
      "normalized": "form_helper_text",
      "source": "archives/mui-packages/mui-material/src/FormHelperText",
      "declared_in": "archives/mui-packages/mui-material/src/FormHelperText/index.js"
    },
    {
      "name": "FormLabel",
      "normalized": "form_label",
      "source": "archives/mui-packages/mui-material/src/FormLabel",
      "declared_in": "archives/mui-packages/mui-material/src/FormLabel/index.js"
    },
    {
      "name": "GlobalStyles",
      "normalized": "global_styles",
      "source": "archives/mui-packages/mui-material/src/GlobalStyles",
      "declared_in": "archives/mui-packages/mui-material/src/GlobalStyles/index.js"
    },
    {
      "name": "Grid",
      "normalized": "grid",
      "source": "archives/mui-packages/mui-material/src/Grid",
      "declared_in": "archives/mui-packages/mui-material/src/Grid/index.ts"
    },
    {
      "name": "Grow",
      "normalized": "grow",
      "source": "archives/mui-packages/mui-material/src/Grow",
      "declared_in": "archives/mui-packages/mui-material/src/Grow/index.js"
    },
    {
      "name": "Icon",
      "normalized": "icon",
      "source": "archives/mui-packages/mui-material/src/Icon",
      "declared_in": "archives/mui-packages/mui-material/src/Icon/index.js"
    },
    {
      "name": "IconButton",
      "normalized": "icon_button",
      "source": "archives/mui-packages/mui-material/src/IconButton",
      "declared_in": "archives/mui-packages/mui-material/src/IconButton/index.js"
    },
    {
      "name": "ImageList",
      "normalized": "image_list",
      "source": "archives/mui-packages/mui-material/src/ImageList",
      "declared_in": "archives/mui-packages/mui-material/src/ImageList/index.js"
    },
    {
      "name": "ImageListItem",
      "normalized": "image_list_item",
      "source": "archives/mui-packages/mui-material/src/ImageListItem",
      "declared_in": "archives/mui-packages/mui-material/src/ImageListItem/index.js"
    },
    {
      "name": "ImageListItemBar",
      "normalized": "image_list_item_bar",
      "source": "archives/mui-packages/mui-material/src/ImageListItemBar",
      "declared_in": "archives/mui-packages/mui-material/src/ImageListItemBar/index.js"
    },
    {
      "name": "InitColorSchemeScript",
      "normalized": "init_color_scheme_script",
      "source": "archives/mui-packages/mui-material/src/InitColorSchemeScript",
      "declared_in": "archives/mui-packages/mui-material/src/InitColorSchemeScript/index.ts"
    },
    {
      "name": "Input",
      "normalized": "input",
      "source": "archives/mui-packages/mui-material/src/Input",
      "declared_in": "archives/mui-packages/mui-material/src/Input/index.js"
    },
    {
      "name": "InputAdornment",
      "normalized": "input_adornment",
      "source": "archives/mui-packages/mui-material/src/InputAdornment",
      "declared_in": "archives/mui-packages/mui-material/src/InputAdornment/index.js"
    },
    {
      "name": "InputBase",
      "normalized": "input_base",
      "source": "archives/mui-packages/mui-material/src/InputBase",
      "declared_in": "archives/mui-packages/mui-material/src/InputBase/index.js"
    },
    {
      "name": "InputLabel",
      "normalized": "input_label",
      "source": "archives/mui-packages/mui-material/src/InputLabel",
      "declared_in": "archives/mui-packages/mui-material/src/InputLabel/index.js"
    },
    {
      "name": "LinearProgress",
      "normalized": "linear_progress",
      "source": "archives/mui-packages/mui-material/src/LinearProgress",
      "declared_in": "archives/mui-packages/mui-material/src/LinearProgress/index.js"
    },
    {
      "name": "Link",
      "normalized": "link",
      "source": "archives/mui-packages/mui-material/src/Link",
      "declared_in": "archives/mui-packages/mui-material/src/Link/index.js"
    },
    {
      "name": "List",
      "normalized": "list",
      "source": "archives/mui-packages/mui-material/src/List",
      "declared_in": "archives/mui-packages/mui-material/src/List/index.js"
    },
    {
      "name": "ListItem",
      "normalized": "list_item",
      "source": "archives/mui-packages/mui-material/src/ListItem",
      "declared_in": "archives/mui-packages/mui-material/src/ListItem/index.js"
    },
    {
      "name": "ListItemAvatar",
      "normalized": "list_item_avatar",
      "source": "archives/mui-packages/mui-material/src/ListItemAvatar",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemAvatar/index.js"
    },
    {
      "name": "ListItemButton",
      "normalized": "list_item_button",
      "source": "archives/mui-packages/mui-material/src/ListItemButton",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemButton/index.js"
    },
    {
      "name": "ListItemIcon",
      "normalized": "list_item_icon",
      "source": "archives/mui-packages/mui-material/src/ListItemIcon",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemIcon/index.js"
    },
    {
      "name": "ListItemSecondaryAction",
      "normalized": "list_item_secondary_action",
      "source": "archives/mui-packages/mui-material/src/ListItemSecondaryAction",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemSecondaryAction/index.js"
    },
    {
      "name": "ListItemText",
      "normalized": "list_item_text",
      "source": "archives/mui-packages/mui-material/src/ListItemText",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemText/index.js"
    },
    {
      "name": "ListSubheader",
      "normalized": "list_subheader",
      "source": "archives/mui-packages/mui-material/src/ListSubheader",
      "declared_in": "archives/mui-packages/mui-material/src/ListSubheader/index.js"
    },
    {
      "name": "Menu",
      "normalized": "menu",
      "source": "archives/mui-packages/mui-material/src/Menu",
      "declared_in": "archives/mui-packages/mui-material/src/Menu/index.js"
    },
    {
      "name": "MenuItem",
      "normalized": "menu_item",
      "source": "archives/mui-packages/mui-material/src/MenuItem",
      "declared_in": "archives/mui-packages/mui-material/src/MenuItem/index.js"
    },
    {
      "name": "MenuList",
      "normalized": "menu_list",
      "source": "archives/mui-packages/mui-material/src/MenuList",
      "declared_in": "archives/mui-packages/mui-material/src/MenuList/index.js"
    },
    {
      "name": "MobileStepper",
      "normalized": "mobile_stepper",
      "source": "archives/mui-packages/mui-material/src/MobileStepper",
      "declared_in": "archives/mui-packages/mui-material/src/MobileStepper/index.js"
    },
    {
      "name": "Modal",
      "normalized": "modal",
      "source": "archives/mui-packages/mui-material/src/Modal",
      "declared_in": "archives/mui-packages/mui-material/src/Modal/index.js"
    },
    {
      "name": "NativeSelect",
      "normalized": "native_select",
      "source": "archives/mui-packages/mui-material/src/NativeSelect",
      "declared_in": "archives/mui-packages/mui-material/src/NativeSelect/index.js"
    },
    {
      "name": "NoSsr",
      "normalized": "no_ssr",
      "source": "archives/mui-packages/mui-material/src/NoSsr",
      "declared_in": "archives/mui-packages/mui-material/src/NoSsr/index.js"
    },
    {
      "name": "OutlinedInput",
      "normalized": "outlined_input",
      "source": "archives/mui-packages/mui-material/src/OutlinedInput",
      "declared_in": "archives/mui-packages/mui-material/src/OutlinedInput/index.js"
    },
    {
      "name": "Pagination",
      "normalized": "pagination",
      "source": "archives/mui-packages/mui-material/src/Pagination",
      "declared_in": "archives/mui-packages/mui-material/src/Pagination/index.js"
    },
    {
      "name": "PaginationItem",
      "normalized": "pagination_item",
      "source": "archives/mui-packages/mui-material/src/PaginationItem",
      "declared_in": "archives/mui-packages/mui-material/src/PaginationItem/index.js"
    },
    {
      "name": "Paper",
      "normalized": "paper",
      "source": "archives/mui-packages/mui-material/src/Paper",
      "declared_in": "archives/mui-packages/mui-material/src/Paper/index.js"
    },
    {
      "name": "PigmentContainer",
      "normalized": "pigment_container",
      "source": "archives/mui-packages/mui-material/src/PigmentContainer",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentContainer/index.ts"
    },
    {
      "name": "PigmentGrid",
      "normalized": "pigment_grid",
      "source": "archives/mui-packages/mui-material/src/PigmentGrid",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentGrid/index.ts"
    },
    {
      "name": "PigmentStack",
      "normalized": "pigment_stack",
      "source": "archives/mui-packages/mui-material/src/PigmentStack",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentStack/index.ts"
    },
    {
      "name": "Popover",
      "normalized": "popover",
      "source": "archives/mui-packages/mui-material/src/Popover",
      "declared_in": "archives/mui-packages/mui-material/src/Popover/index.js"
    },
    {
      "name": "Popper",
      "normalized": "popper",
      "source": "archives/mui-packages/mui-material/src/Popper",
      "declared_in": "archives/mui-packages/mui-material/src/Popper/index.js"
    },
    {
      "name": "Portal",
      "normalized": "portal",
      "source": "archives/mui-packages/mui-material/src/Portal",
      "declared_in": "archives/mui-packages/mui-material/src/Portal/index.js"
    },
    {
      "name": "Radio",
      "normalized": "radio",
      "source": "archives/mui-packages/mui-material/src/Radio",
      "declared_in": "archives/mui-packages/mui-material/src/Radio/index.js"
    },
    {
      "name": "RadioGroup",
      "normalized": "radio_group",
      "source": "archives/mui-packages/mui-material/src/RadioGroup",
      "declared_in": "archives/mui-packages/mui-material/src/RadioGroup/index.js"
    },
    {
      "name": "Rating",
      "normalized": "rating",
      "source": "archives/mui-packages/mui-material/src/Rating",
      "declared_in": "archives/mui-packages/mui-material/src/Rating/index.js"
    },
    {
      "name": "ScopedCssBaseline",
      "normalized": "scoped_css_baseline",
      "source": "archives/mui-packages/mui-material/src/ScopedCssBaseline",
      "declared_in": "archives/mui-packages/mui-material/src/ScopedCssBaseline/index.js"
    },
    {
      "name": "Select",
      "normalized": "select",
      "source": "archives/mui-packages/mui-material/src/Select",
      "declared_in": "archives/mui-packages/mui-material/src/Select/index.js"
    },
    {
      "name": "Skeleton",
      "normalized": "skeleton",
      "source": "archives/mui-packages/mui-material/src/Skeleton",
      "declared_in": "archives/mui-packages/mui-material/src/Skeleton/index.js"
    },
    {
      "name": "Slide",
      "normalized": "slide",
      "source": "archives/mui-packages/mui-material/src/Slide",
      "declared_in": "archives/mui-packages/mui-material/src/Slide/index.js"
    },
    {
      "name": "Slider",
      "normalized": "slider",
      "source": "archives/mui-packages/mui-material/src/Slider",
      "declared_in": "archives/mui-packages/mui-material/src/Slider/index.js"
    },
    {
      "name": "Snackbar",
      "normalized": "snackbar",
      "source": "archives/mui-packages/mui-material/src/Snackbar",
      "declared_in": "archives/mui-packages/mui-material/src/Snackbar/index.js"
    },
    {
      "name": "SnackbarContent",
      "normalized": "snackbar_content",
      "source": "archives/mui-packages/mui-material/src/SnackbarContent",
      "declared_in": "archives/mui-packages/mui-material/src/SnackbarContent/index.js"
    },
    {
      "name": "SpeedDial",
      "normalized": "speed_dial",
      "source": "archives/mui-packages/mui-material/src/SpeedDial",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDial/index.js"
    },
    {
      "name": "SpeedDialAction",
      "normalized": "speed_dial_action",
      "source": "archives/mui-packages/mui-material/src/SpeedDialAction",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDialAction/index.js"
    },
    {
      "name": "SpeedDialIcon",
      "normalized": "speed_dial_icon",
      "source": "archives/mui-packages/mui-material/src/SpeedDialIcon",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDialIcon/index.js"
    },
    {
      "name": "Stack",
      "normalized": "stack",
      "source": "archives/mui-packages/mui-material/src/Stack",
      "declared_in": "archives/mui-packages/mui-material/src/Stack/index.js"
    },
    {
      "name": "Step",
      "normalized": "step",
      "source": "archives/mui-packages/mui-material/src/Step",
      "declared_in": "archives/mui-packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepButton",
      "normalized": "step_button",
      "source": "archives/mui-packages/mui-material/src/StepButton",
      "declared_in": "archives/mui-packages/mui-material/src/StepButton/index.js"
    },
    {
      "name": "StepConnector",
      "normalized": "step_connector",
      "source": "archives/mui-packages/mui-material/src/StepConnector",
      "declared_in": "archives/mui-packages/mui-material/src/StepConnector/index.js"
    },
    {
      "name": "StepContent",
      "normalized": "step_content",
      "source": "archives/mui-packages/mui-material/src/StepContent",
      "declared_in": "archives/mui-packages/mui-material/src/StepContent/index.js"
    },
    {
      "name": "StepContext",
      "normalized": "step_context",
      "source": "archives/mui-packages/mui-material/src/StepContext",
      "declared_in": "archives/mui-packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepIcon",
      "normalized": "step_icon",
      "source": "archives/mui-packages/mui-material/src/StepIcon",
      "declared_in": "archives/mui-packages/mui-material/src/StepIcon/index.js"
    },
    {
      "name": "StepLabel",
      "normalized": "step_label",
      "source": "archives/mui-packages/mui-material/src/StepLabel",
      "declared_in": "archives/mui-packages/mui-material/src/StepLabel/index.js"
    },
    {
      "name": "Stepper",
      "normalized": "stepper",
      "source": "archives/mui-packages/mui-material/src/Stepper",
      "declared_in": "archives/mui-packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "StepperContext",
      "normalized": "stepper_context",
      "source": "archives/mui-packages/mui-material/src/StepperContext",
      "declared_in": "archives/mui-packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "SvgIcon",
      "normalized": "svg_icon",
      "source": "archives/mui-packages/mui-material/src/SvgIcon",
      "declared_in": "archives/mui-packages/mui-material/src/SvgIcon/index.js"
    },
    {
      "name": "SwipeableDrawer",
      "normalized": "swipeable_drawer",
      "source": "archives/mui-packages/mui-material/src/SwipeableDrawer",
      "declared_in": "archives/mui-packages/mui-material/src/SwipeableDrawer/index.js"
    },
    {
      "name": "Switch",
      "normalized": "switch",
      "source": "archives/mui-packages/mui-material/src/Switch",
      "declared_in": "archives/mui-packages/mui-material/src/Switch/index.js"
    },
    {
      "name": "Tab",
      "normalized": "tab",
      "source": "archives/mui-packages/mui-material/src/Tab",
      "declared_in": "archives/mui-packages/mui-material/src/Tab/index.js"
    },
    {
      "name": "TabScrollButton",
      "normalized": "tab_scroll_button",
      "source": "archives/mui-packages/mui-material/src/TabScrollButton",
      "declared_in": "archives/mui-packages/mui-material/src/TabScrollButton/index.js"
    },
    {
      "name": "Table",
      "normalized": "table",
      "source": "archives/mui-packages/mui-material/src/Table",
      "declared_in": "archives/mui-packages/mui-material/src/Table/index.js"
    },
    {
      "name": "TableBody",
      "normalized": "table_body",
      "source": "archives/mui-packages/mui-material/src/TableBody",
      "declared_in": "archives/mui-packages/mui-material/src/TableBody/index.js"
    },
    {
      "name": "TableCell",
      "normalized": "table_cell",
      "source": "archives/mui-packages/mui-material/src/TableCell",
      "declared_in": "archives/mui-packages/mui-material/src/TableCell/index.js"
    },
    {
      "name": "TableContainer",
      "normalized": "table_container",
      "source": "archives/mui-packages/mui-material/src/TableContainer",
      "declared_in": "archives/mui-packages/mui-material/src/TableContainer/index.js"
    },
    {
      "name": "TableFooter",
      "normalized": "table_footer",
      "source": "archives/mui-packages/mui-material/src/TableFooter",
      "declared_in": "archives/mui-packages/mui-material/src/TableFooter/index.js"
    },
    {
      "name": "TableHead",
      "normalized": "table_head",
      "source": "archives/mui-packages/mui-material/src/TableHead",
      "declared_in": "archives/mui-packages/mui-material/src/TableHead/index.js"
    },
    {
      "name": "TablePagination",
      "normalized": "table_pagination",
      "source": "archives/mui-packages/mui-material/src/TablePagination",
      "declared_in": "archives/mui-packages/mui-material/src/TablePagination/index.js"
    },
    {
      "name": "TablePaginationActions",
      "normalized": "table_pagination_actions",
      "source": "archives/mui-packages/mui-material/src/TablePaginationActions",
      "declared_in": "archives/mui-packages/mui-material/src/TablePaginationActions/index.js"
    },
    {
      "name": "TableRow",
      "normalized": "table_row",
      "source": "archives/mui-packages/mui-material/src/TableRow",
      "declared_in": "archives/mui-packages/mui-material/src/TableRow/index.js"
    },
    {
      "name": "TableSortLabel",
      "normalized": "table_sort_label",
      "source": "archives/mui-packages/mui-material/src/TableSortLabel",
      "declared_in": "archives/mui-packages/mui-material/src/TableSortLabel/index.js"
    },
    {
      "name": "Tabs",
      "normalized": "tabs",
      "source": "archives/mui-packages/mui-material/src/Tabs",
      "declared_in": "archives/mui-packages/mui-material/src/Tabs/index.js"
    },
    {
      "name": "TextField",
      "normalized": "text_field",
      "source": "archives/mui-packages/mui-material/src/TextField",
      "declared_in": "archives/mui-packages/mui-material/src/TextField/index.js"
    },
    {
      "name": "TextareaAutosize",
      "normalized": "textarea_autosize",
      "source": "archives/mui-packages/mui-material/src/TextareaAutosize",
      "declared_in": "archives/mui-packages/mui-material/src/TextareaAutosize/index.js"
    },
    {
      "name": "THEME_ID",
      "normalized": "theme_id",
      "source": "archives/mui-packages/mui-material/src/identifier",
      "declared_in": "archives/mui-packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ThemeProvider",
      "normalized": "theme_provider",
      "source": "archives/mui-packages/mui-material/src/ThemeProvider",
      "declared_in": "archives/mui-packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ToggleButton",
      "normalized": "toggle_button",
      "source": "archives/mui-packages/mui-material/src/ToggleButton",
      "declared_in": "archives/mui-packages/mui-material/src/ToggleButton/index.js"
    },
    {
      "name": "ToggleButtonGroup",
      "normalized": "toggle_button_group",
      "source": "archives/mui-packages/mui-material/src/ToggleButtonGroup",
      "declared_in": "archives/mui-packages/mui-material/src/ToggleButtonGroup/index.js"
    },
    {
      "name": "Toolbar",
      "normalized": "toolbar",
      "source": "archives/mui-packages/mui-material/src/Toolbar",
      "declared_in": "archives/mui-packages/mui-material/src/Toolbar/index.js"
    },
    {
      "name": "Tooltip",
      "normalized": "tooltip",
      "source": "archives/mui-packages/mui-material/src/Tooltip",
      "declared_in": "archives/mui-packages/mui-material/src/Tooltip/index.js"
    },
    {
      "name": "Typography",
      "normalized": "typography",
      "source": "archives/mui-packages/mui-material/src/Typography",
      "declared_in": "archives/mui-packages/mui-material/src/Typography/index.js"
    },
    {
      "name": "Unstable_TrapFocus",
      "normalized": "unstable_trap_focus",
      "source": "archives/mui-packages/mui-material/src/Unstable_TrapFocus",
      "declared_in": "archives/mui-packages/mui-material/src/index.js"
    },
    {
      "name": "UseAutocomplete",
      "normalized": "use_autocomplete",
      "source": "archives/mui-packages/mui-material/src/useAutocomplete",
      "declared_in": "archives/mui-packages/mui-material/src/useAutocomplete/index.js"
    },
    {
      "name": "UseLazyRipple",
      "normalized": "use_lazy_ripple",
      "source": "archives/mui-packages/mui-material/src/useLazyRipple",
      "declared_in": "archives/mui-packages/mui-material/src/useLazyRipple/index.ts"
    },
    {
      "name": "UsePagination",
      "normalized": "use_pagination",
      "source": "archives/mui-packages/mui-material/src/usePagination",
      "declared_in": "archives/mui-packages/mui-material/src/usePagination/index.js"
    },
    {
      "name": "UseScrollTrigger",
      "normalized": "use_scroll_trigger",
      "source": "archives/mui-packages/mui-material/src/useScrollTrigger",
      "declared_in": "archives/mui-packages/mui-material/src/useScrollTrigger/index.js"
    },
    {
      "name": "Zoom",
      "normalized": "zoom",
      "source": "archives/mui-packages/mui-material/src/Zoom",
      "declared_in": "archives/mui-packages/mui-material/src/Zoom/index.js"
    }
  ],
  "missing_from_material": [
    {
      "name": "Accordion",
      "normalized": "accordion",
      "source": "archives/mui-packages/mui-material/src/Accordion",
      "declared_in": "archives/mui-packages/mui-material/src/Accordion/index.js"
    },
    {
      "name": "AccordionActions",
      "normalized": "accordion_actions",
      "source": "archives/mui-packages/mui-material/src/AccordionActions",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionActions/index.js"
    },
    {
      "name": "AccordionDetails",
      "normalized": "accordion_details",
      "source": "archives/mui-packages/mui-material/src/AccordionDetails",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionDetails/index.js"
    },
    {
      "name": "AccordionSummary",
      "normalized": "accordion_summary",
      "source": "archives/mui-packages/mui-material/src/AccordionSummary",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionSummary/index.js"
    },
    {
      "name": "Alert",
      "normalized": "alert",
      "source": "archives/mui-packages/mui-material/src/Alert",
      "declared_in": "archives/mui-packages/mui-material/src/Alert/index.js"
    },
    {
      "name": "AlertTitle",
      "normalized": "alert_title",
      "source": "archives/mui-packages/mui-material/src/AlertTitle",
      "declared_in": "archives/mui-packages/mui-material/src/AlertTitle/index.js"
    },
    {
      "name": "Autocomplete",
      "normalized": "autocomplete",
      "source": "archives/mui-packages/mui-material/src/Autocomplete",
      "declared_in": "archives/mui-packages/mui-material/src/Autocomplete/index.js"
    },
    {
      "name": "Avatar",
      "normalized": "avatar",
      "source": "archives/mui-packages/mui-material/src/Avatar",
      "declared_in": "archives/mui-packages/mui-material/src/Avatar/index.js"
    },
    {
      "name": "AvatarGroup",
      "normalized": "avatar_group",
      "source": "archives/mui-packages/mui-material/src/AvatarGroup",
      "declared_in": "archives/mui-packages/mui-material/src/AvatarGroup/index.js"
    },
    {
      "name": "Backdrop",
      "normalized": "backdrop",
      "source": "archives/mui-packages/mui-material/src/Backdrop",
      "declared_in": "archives/mui-packages/mui-material/src/Backdrop/index.js"
    },
    {
      "name": "Badge",
      "normalized": "badge",
      "source": "archives/mui-packages/mui-material/src/Badge",
      "declared_in": "archives/mui-packages/mui-material/src/Badge/index.js"
    },
    {
      "name": "BottomNavigation",
      "normalized": "bottom_navigation",
      "source": "archives/mui-packages/mui-material/src/BottomNavigation",
      "declared_in": "archives/mui-packages/mui-material/src/BottomNavigation/index.js"
    },
    {
      "name": "BottomNavigationAction",
      "normalized": "bottom_navigation_action",
      "source": "archives/mui-packages/mui-material/src/BottomNavigationAction",
      "declared_in": "archives/mui-packages/mui-material/src/BottomNavigationAction/index.js"
    },
    {
      "name": "Box",
      "normalized": "box",
      "source": "archives/mui-packages/mui-material/src/Box",
      "declared_in": "archives/mui-packages/mui-material/src/Box/index.js"
    },
    {
      "name": "Breadcrumbs",
      "normalized": "breadcrumbs",
      "source": "archives/mui-packages/mui-material/src/Breadcrumbs",
      "declared_in": "archives/mui-packages/mui-material/src/Breadcrumbs/index.js"
    },
    {
      "name": "ButtonBase",
      "normalized": "button_base",
      "source": "archives/mui-packages/mui-material/src/ButtonBase",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonBase/index.js"
    },
    {
      "name": "ButtonGroup",
      "normalized": "button_group",
      "source": "archives/mui-packages/mui-material/src/ButtonGroup",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupButtonContext",
      "normalized": "button_group_button_context",
      "source": "archives/mui-packages/mui-material/src/ButtonGroupButtonContext",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupContext",
      "normalized": "button_group_context",
      "source": "archives/mui-packages/mui-material/src/ButtonGroupContext",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "CardActionArea",
      "normalized": "card_action_area",
      "source": "archives/mui-packages/mui-material/src/CardActionArea",
      "declared_in": "archives/mui-packages/mui-material/src/CardActionArea/index.js"
    },
    {
      "name": "CardActions",
      "normalized": "card_actions",
      "source": "archives/mui-packages/mui-material/src/CardActions",
      "declared_in": "archives/mui-packages/mui-material/src/CardActions/index.js"
    },
    {
      "name": "CardContent",
      "normalized": "card_content",
      "source": "archives/mui-packages/mui-material/src/CardContent",
      "declared_in": "archives/mui-packages/mui-material/src/CardContent/index.js"
    },
    {
      "name": "CardHeader",
      "normalized": "card_header",
      "source": "archives/mui-packages/mui-material/src/CardHeader",
      "declared_in": "archives/mui-packages/mui-material/src/CardHeader/index.js"
    },
    {
      "name": "CardMedia",
      "normalized": "card_media",
      "source": "archives/mui-packages/mui-material/src/CardMedia",
      "declared_in": "archives/mui-packages/mui-material/src/CardMedia/index.js"
    },
    {
      "name": "Checkbox",
      "normalized": "checkbox",
      "source": "archives/mui-packages/mui-material/src/Checkbox",
      "declared_in": "archives/mui-packages/mui-material/src/Checkbox/index.js"
    },
    {
      "name": "Chip",
      "normalized": "chip",
      "source": "archives/mui-packages/mui-material/src/Chip",
      "declared_in": "archives/mui-packages/mui-material/src/Chip/index.js"
    },
    {
      "name": "CircularProgress",
      "normalized": "circular_progress",
      "source": "archives/mui-packages/mui-material/src/CircularProgress",
      "declared_in": "archives/mui-packages/mui-material/src/CircularProgress/index.js"
    },
    {
      "name": "ClickAwayListener",
      "normalized": "click_away_listener",
      "source": "archives/mui-packages/mui-material/src/ClickAwayListener",
      "declared_in": "archives/mui-packages/mui-material/src/index.js"
    },
    {
      "name": "Collapse",
      "normalized": "collapse",
      "source": "archives/mui-packages/mui-material/src/Collapse",
      "declared_in": "archives/mui-packages/mui-material/src/Collapse/index.js"
    },
    {
      "name": "Container",
      "normalized": "container",
      "source": "archives/mui-packages/mui-material/src/Container",
      "declared_in": "archives/mui-packages/mui-material/src/Container/index.js"
    },
    {
      "name": "CssBaseline",
      "normalized": "css_baseline",
      "source": "archives/mui-packages/mui-material/src/CssBaseline",
      "declared_in": "archives/mui-packages/mui-material/src/CssBaseline/index.js"
    },
    {
      "name": "DefaultPropsProvider",
      "normalized": "default_props_provider",
      "source": "archives/mui-packages/mui-material/src/DefaultPropsProvider",
      "declared_in": "archives/mui-packages/mui-material/src/DefaultPropsProvider/index.ts"
    },
    {
      "name": "DialogActions",
      "normalized": "dialog_actions",
      "source": "archives/mui-packages/mui-material/src/DialogActions",
      "declared_in": "archives/mui-packages/mui-material/src/DialogActions/index.js"
    },
    {
      "name": "DialogContent",
      "normalized": "dialog_content",
      "source": "archives/mui-packages/mui-material/src/DialogContent",
      "declared_in": "archives/mui-packages/mui-material/src/DialogContent/index.js"
    },
    {
      "name": "DialogContentText",
      "normalized": "dialog_content_text",
      "source": "archives/mui-packages/mui-material/src/DialogContentText",
      "declared_in": "archives/mui-packages/mui-material/src/DialogContentText/index.js"
    },
    {
      "name": "DialogTitle",
      "normalized": "dialog_title",
      "source": "archives/mui-packages/mui-material/src/DialogTitle",
      "declared_in": "archives/mui-packages/mui-material/src/DialogTitle/index.js"
    },
    {
      "name": "Divider",
      "normalized": "divider",
      "source": "archives/mui-packages/mui-material/src/Divider",
      "declared_in": "archives/mui-packages/mui-material/src/Divider/index.js"
    },
    {
      "name": "Drawer",
      "normalized": "drawer",
      "source": "archives/mui-packages/mui-material/src/Drawer",
      "declared_in": "archives/mui-packages/mui-material/src/Drawer/index.js"
    },
    {
      "name": "Fab",
      "normalized": "fab",
      "source": "archives/mui-packages/mui-material/src/Fab",
      "declared_in": "archives/mui-packages/mui-material/src/Fab/index.js"
    },
    {
      "name": "Fade",
      "normalized": "fade",
      "source": "archives/mui-packages/mui-material/src/Fade",
      "declared_in": "archives/mui-packages/mui-material/src/Fade/index.js"
    },
    {
      "name": "FilledInput",
      "normalized": "filled_input",
      "source": "archives/mui-packages/mui-material/src/FilledInput",
      "declared_in": "archives/mui-packages/mui-material/src/FilledInput/index.js"
    },
    {
      "name": "FocusTrap",
      "normalized": "focus_trap",
      "source": "archives/mui-packages/mui-material/src/FocusTrap",
      "declared_in": "archives/mui-packages/mui-material/src/Unstable_TrapFocus/index.js"
    },
    {
      "name": "FormControl",
      "normalized": "form_control",
      "source": "archives/mui-packages/mui-material/src/FormControl",
      "declared_in": "archives/mui-packages/mui-material/src/FormControl/index.js"
    },
    {
      "name": "FormControlLabel",
      "normalized": "form_control_label",
      "source": "archives/mui-packages/mui-material/src/FormControlLabel",
      "declared_in": "archives/mui-packages/mui-material/src/FormControlLabel/index.js"
    },
    {
      "name": "FormGroup",
      "normalized": "form_group",
      "source": "archives/mui-packages/mui-material/src/FormGroup",
      "declared_in": "archives/mui-packages/mui-material/src/FormGroup/index.js"
    },
    {
      "name": "FormHelperText",
      "normalized": "form_helper_text",
      "source": "archives/mui-packages/mui-material/src/FormHelperText",
      "declared_in": "archives/mui-packages/mui-material/src/FormHelperText/index.js"
    },
    {
      "name": "FormLabel",
      "normalized": "form_label",
      "source": "archives/mui-packages/mui-material/src/FormLabel",
      "declared_in": "archives/mui-packages/mui-material/src/FormLabel/index.js"
    },
    {
      "name": "GlobalStyles",
      "normalized": "global_styles",
      "source": "archives/mui-packages/mui-material/src/GlobalStyles",
      "declared_in": "archives/mui-packages/mui-material/src/GlobalStyles/index.js"
    },
    {
      "name": "Grid",
      "normalized": "grid",
      "source": "archives/mui-packages/mui-material/src/Grid",
      "declared_in": "archives/mui-packages/mui-material/src/Grid/index.ts"
    },
    {
      "name": "Grow",
      "normalized": "grow",
      "source": "archives/mui-packages/mui-material/src/Grow",
      "declared_in": "archives/mui-packages/mui-material/src/Grow/index.js"
    },
    {
      "name": "Icon",
      "normalized": "icon",
      "source": "archives/mui-packages/mui-material/src/Icon",
      "declared_in": "archives/mui-packages/mui-material/src/Icon/index.js"
    },
    {
      "name": "IconButton",
      "normalized": "icon_button",
      "source": "archives/mui-packages/mui-material/src/IconButton",
      "declared_in": "archives/mui-packages/mui-material/src/IconButton/index.js"
    },
    {
      "name": "ImageList",
      "normalized": "image_list",
      "source": "archives/mui-packages/mui-material/src/ImageList",
      "declared_in": "archives/mui-packages/mui-material/src/ImageList/index.js"
    },
    {
      "name": "ImageListItem",
      "normalized": "image_list_item",
      "source": "archives/mui-packages/mui-material/src/ImageListItem",
      "declared_in": "archives/mui-packages/mui-material/src/ImageListItem/index.js"
    },
    {
      "name": "ImageListItemBar",
      "normalized": "image_list_item_bar",
      "source": "archives/mui-packages/mui-material/src/ImageListItemBar",
      "declared_in": "archives/mui-packages/mui-material/src/ImageListItemBar/index.js"
    },
    {
      "name": "InitColorSchemeScript",
      "normalized": "init_color_scheme_script",
      "source": "archives/mui-packages/mui-material/src/InitColorSchemeScript",
      "declared_in": "archives/mui-packages/mui-material/src/InitColorSchemeScript/index.ts"
    },
    {
      "name": "Input",
      "normalized": "input",
      "source": "archives/mui-packages/mui-material/src/Input",
      "declared_in": "archives/mui-packages/mui-material/src/Input/index.js"
    },
    {
      "name": "InputAdornment",
      "normalized": "input_adornment",
      "source": "archives/mui-packages/mui-material/src/InputAdornment",
      "declared_in": "archives/mui-packages/mui-material/src/InputAdornment/index.js"
    },
    {
      "name": "InputBase",
      "normalized": "input_base",
      "source": "archives/mui-packages/mui-material/src/InputBase",
      "declared_in": "archives/mui-packages/mui-material/src/InputBase/index.js"
    },
    {
      "name": "InputLabel",
      "normalized": "input_label",
      "source": "archives/mui-packages/mui-material/src/InputLabel",
      "declared_in": "archives/mui-packages/mui-material/src/InputLabel/index.js"
    },
    {
      "name": "LinearProgress",
      "normalized": "linear_progress",
      "source": "archives/mui-packages/mui-material/src/LinearProgress",
      "declared_in": "archives/mui-packages/mui-material/src/LinearProgress/index.js"
    },
    {
      "name": "Link",
      "normalized": "link",
      "source": "archives/mui-packages/mui-material/src/Link",
      "declared_in": "archives/mui-packages/mui-material/src/Link/index.js"
    },
    {
      "name": "List",
      "normalized": "list",
      "source": "archives/mui-packages/mui-material/src/List",
      "declared_in": "archives/mui-packages/mui-material/src/List/index.js"
    },
    {
      "name": "ListItem",
      "normalized": "list_item",
      "source": "archives/mui-packages/mui-material/src/ListItem",
      "declared_in": "archives/mui-packages/mui-material/src/ListItem/index.js"
    },
    {
      "name": "ListItemAvatar",
      "normalized": "list_item_avatar",
      "source": "archives/mui-packages/mui-material/src/ListItemAvatar",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemAvatar/index.js"
    },
    {
      "name": "ListItemButton",
      "normalized": "list_item_button",
      "source": "archives/mui-packages/mui-material/src/ListItemButton",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemButton/index.js"
    },
    {
      "name": "ListItemIcon",
      "normalized": "list_item_icon",
      "source": "archives/mui-packages/mui-material/src/ListItemIcon",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemIcon/index.js"
    },
    {
      "name": "ListItemSecondaryAction",
      "normalized": "list_item_secondary_action",
      "source": "archives/mui-packages/mui-material/src/ListItemSecondaryAction",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemSecondaryAction/index.js"
    },
    {
      "name": "ListItemText",
      "normalized": "list_item_text",
      "source": "archives/mui-packages/mui-material/src/ListItemText",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemText/index.js"
    },
    {
      "name": "ListSubheader",
      "normalized": "list_subheader",
      "source": "archives/mui-packages/mui-material/src/ListSubheader",
      "declared_in": "archives/mui-packages/mui-material/src/ListSubheader/index.js"
    },
    {
      "name": "Menu",
      "normalized": "menu",
      "source": "archives/mui-packages/mui-material/src/Menu",
      "declared_in": "archives/mui-packages/mui-material/src/Menu/index.js"
    },
    {
      "name": "MenuItem",
      "normalized": "menu_item",
      "source": "archives/mui-packages/mui-material/src/MenuItem",
      "declared_in": "archives/mui-packages/mui-material/src/MenuItem/index.js"
    },
    {
      "name": "MenuList",
      "normalized": "menu_list",
      "source": "archives/mui-packages/mui-material/src/MenuList",
      "declared_in": "archives/mui-packages/mui-material/src/MenuList/index.js"
    },
    {
      "name": "MobileStepper",
      "normalized": "mobile_stepper",
      "source": "archives/mui-packages/mui-material/src/MobileStepper",
      "declared_in": "archives/mui-packages/mui-material/src/MobileStepper/index.js"
    },
    {
      "name": "Modal",
      "normalized": "modal",
      "source": "archives/mui-packages/mui-material/src/Modal",
      "declared_in": "archives/mui-packages/mui-material/src/Modal/index.js"
    },
    {
      "name": "NativeSelect",
      "normalized": "native_select",
      "source": "archives/mui-packages/mui-material/src/NativeSelect",
      "declared_in": "archives/mui-packages/mui-material/src/NativeSelect/index.js"
    },
    {
      "name": "NoSsr",
      "normalized": "no_ssr",
      "source": "archives/mui-packages/mui-material/src/NoSsr",
      "declared_in": "archives/mui-packages/mui-material/src/NoSsr/index.js"
    },
    {
      "name": "OutlinedInput",
      "normalized": "outlined_input",
      "source": "archives/mui-packages/mui-material/src/OutlinedInput",
      "declared_in": "archives/mui-packages/mui-material/src/OutlinedInput/index.js"
    },
    {
      "name": "Pagination",
      "normalized": "pagination",
      "source": "archives/mui-packages/mui-material/src/Pagination",
      "declared_in": "archives/mui-packages/mui-material/src/Pagination/index.js"
    },
    {
      "name": "PaginationItem",
      "normalized": "pagination_item",
      "source": "archives/mui-packages/mui-material/src/PaginationItem",
      "declared_in": "archives/mui-packages/mui-material/src/PaginationItem/index.js"
    },
    {
      "name": "Paper",
      "normalized": "paper",
      "source": "archives/mui-packages/mui-material/src/Paper",
      "declared_in": "archives/mui-packages/mui-material/src/Paper/index.js"
    },
    {
      "name": "PigmentContainer",
      "normalized": "pigment_container",
      "source": "archives/mui-packages/mui-material/src/PigmentContainer",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentContainer/index.ts"
    },
    {
      "name": "PigmentGrid",
      "normalized": "pigment_grid",
      "source": "archives/mui-packages/mui-material/src/PigmentGrid",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentGrid/index.ts"
    },
    {
      "name": "PigmentStack",
      "normalized": "pigment_stack",
      "source": "archives/mui-packages/mui-material/src/PigmentStack",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentStack/index.ts"
    },
    {
      "name": "Popover",
      "normalized": "popover",
      "source": "archives/mui-packages/mui-material/src/Popover",
      "declared_in": "archives/mui-packages/mui-material/src/Popover/index.js"
    },
    {
      "name": "Popper",
      "normalized": "popper",
      "source": "archives/mui-packages/mui-material/src/Popper",
      "declared_in": "archives/mui-packages/mui-material/src/Popper/index.js"
    },
    {
      "name": "Portal",
      "normalized": "portal",
      "source": "archives/mui-packages/mui-material/src/Portal",
      "declared_in": "archives/mui-packages/mui-material/src/Portal/index.js"
    },
    {
      "name": "Radio",
      "normalized": "radio",
      "source": "archives/mui-packages/mui-material/src/Radio",
      "declared_in": "archives/mui-packages/mui-material/src/Radio/index.js"
    },
    {
      "name": "RadioGroup",
      "normalized": "radio_group",
      "source": "archives/mui-packages/mui-material/src/RadioGroup",
      "declared_in": "archives/mui-packages/mui-material/src/RadioGroup/index.js"
    },
    {
      "name": "Rating",
      "normalized": "rating",
      "source": "archives/mui-packages/mui-material/src/Rating",
      "declared_in": "archives/mui-packages/mui-material/src/Rating/index.js"
    },
    {
      "name": "ScopedCssBaseline",
      "normalized": "scoped_css_baseline",
      "source": "archives/mui-packages/mui-material/src/ScopedCssBaseline",
      "declared_in": "archives/mui-packages/mui-material/src/ScopedCssBaseline/index.js"
    },
    {
      "name": "Select",
      "normalized": "select",
      "source": "archives/mui-packages/mui-material/src/Select",
      "declared_in": "archives/mui-packages/mui-material/src/Select/index.js"
    },
    {
      "name": "Skeleton",
      "normalized": "skeleton",
      "source": "archives/mui-packages/mui-material/src/Skeleton",
      "declared_in": "archives/mui-packages/mui-material/src/Skeleton/index.js"
    },
    {
      "name": "Slide",
      "normalized": "slide",
      "source": "archives/mui-packages/mui-material/src/Slide",
      "declared_in": "archives/mui-packages/mui-material/src/Slide/index.js"
    },
    {
      "name": "Slider",
      "normalized": "slider",
      "source": "archives/mui-packages/mui-material/src/Slider",
      "declared_in": "archives/mui-packages/mui-material/src/Slider/index.js"
    },
    {
      "name": "SnackbarContent",
      "normalized": "snackbar_content",
      "source": "archives/mui-packages/mui-material/src/SnackbarContent",
      "declared_in": "archives/mui-packages/mui-material/src/SnackbarContent/index.js"
    },
    {
      "name": "SpeedDial",
      "normalized": "speed_dial",
      "source": "archives/mui-packages/mui-material/src/SpeedDial",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDial/index.js"
    },
    {
      "name": "SpeedDialAction",
      "normalized": "speed_dial_action",
      "source": "archives/mui-packages/mui-material/src/SpeedDialAction",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDialAction/index.js"
    },
    {
      "name": "SpeedDialIcon",
      "normalized": "speed_dial_icon",
      "source": "archives/mui-packages/mui-material/src/SpeedDialIcon",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDialIcon/index.js"
    },
    {
      "name": "Stack",
      "normalized": "stack",
      "source": "archives/mui-packages/mui-material/src/Stack",
      "declared_in": "archives/mui-packages/mui-material/src/Stack/index.js"
    },
    {
      "name": "Step",
      "normalized": "step",
      "source": "archives/mui-packages/mui-material/src/Step",
      "declared_in": "archives/mui-packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepButton",
      "normalized": "step_button",
      "source": "archives/mui-packages/mui-material/src/StepButton",
      "declared_in": "archives/mui-packages/mui-material/src/StepButton/index.js"
    },
    {
      "name": "StepConnector",
      "normalized": "step_connector",
      "source": "archives/mui-packages/mui-material/src/StepConnector",
      "declared_in": "archives/mui-packages/mui-material/src/StepConnector/index.js"
    },
    {
      "name": "StepContent",
      "normalized": "step_content",
      "source": "archives/mui-packages/mui-material/src/StepContent",
      "declared_in": "archives/mui-packages/mui-material/src/StepContent/index.js"
    },
    {
      "name": "StepContext",
      "normalized": "step_context",
      "source": "archives/mui-packages/mui-material/src/StepContext",
      "declared_in": "archives/mui-packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepIcon",
      "normalized": "step_icon",
      "source": "archives/mui-packages/mui-material/src/StepIcon",
      "declared_in": "archives/mui-packages/mui-material/src/StepIcon/index.js"
    },
    {
      "name": "StepLabel",
      "normalized": "step_label",
      "source": "archives/mui-packages/mui-material/src/StepLabel",
      "declared_in": "archives/mui-packages/mui-material/src/StepLabel/index.js"
    },
    {
      "name": "Stepper",
      "normalized": "stepper",
      "source": "archives/mui-packages/mui-material/src/Stepper",
      "declared_in": "archives/mui-packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "StepperContext",
      "normalized": "stepper_context",
      "source": "archives/mui-packages/mui-material/src/StepperContext",
      "declared_in": "archives/mui-packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "SvgIcon",
      "normalized": "svg_icon",
      "source": "archives/mui-packages/mui-material/src/SvgIcon",
      "declared_in": "archives/mui-packages/mui-material/src/SvgIcon/index.js"
    },
    {
      "name": "SwipeableDrawer",
      "normalized": "swipeable_drawer",
      "source": "archives/mui-packages/mui-material/src/SwipeableDrawer",
      "declared_in": "archives/mui-packages/mui-material/src/SwipeableDrawer/index.js"
    },
    {
      "name": "Switch",
      "normalized": "switch",
      "source": "archives/mui-packages/mui-material/src/Switch",
      "declared_in": "archives/mui-packages/mui-material/src/Switch/index.js"
    },
    {
      "name": "Tab",
      "normalized": "tab",
      "source": "archives/mui-packages/mui-material/src/Tab",
      "declared_in": "archives/mui-packages/mui-material/src/Tab/index.js"
    },
    {
      "name": "TabScrollButton",
      "normalized": "tab_scroll_button",
      "source": "archives/mui-packages/mui-material/src/TabScrollButton",
      "declared_in": "archives/mui-packages/mui-material/src/TabScrollButton/index.js"
    },
    {
      "name": "Table",
      "normalized": "table",
      "source": "archives/mui-packages/mui-material/src/Table",
      "declared_in": "archives/mui-packages/mui-material/src/Table/index.js"
    },
    {
      "name": "TableBody",
      "normalized": "table_body",
      "source": "archives/mui-packages/mui-material/src/TableBody",
      "declared_in": "archives/mui-packages/mui-material/src/TableBody/index.js"
    },
    {
      "name": "TableCell",
      "normalized": "table_cell",
      "source": "archives/mui-packages/mui-material/src/TableCell",
      "declared_in": "archives/mui-packages/mui-material/src/TableCell/index.js"
    },
    {
      "name": "TableContainer",
      "normalized": "table_container",
      "source": "archives/mui-packages/mui-material/src/TableContainer",
      "declared_in": "archives/mui-packages/mui-material/src/TableContainer/index.js"
    },
    {
      "name": "TableFooter",
      "normalized": "table_footer",
      "source": "archives/mui-packages/mui-material/src/TableFooter",
      "declared_in": "archives/mui-packages/mui-material/src/TableFooter/index.js"
    },
    {
      "name": "TableHead",
      "normalized": "table_head",
      "source": "archives/mui-packages/mui-material/src/TableHead",
      "declared_in": "archives/mui-packages/mui-material/src/TableHead/index.js"
    },
    {
      "name": "TablePagination",
      "normalized": "table_pagination",
      "source": "archives/mui-packages/mui-material/src/TablePagination",
      "declared_in": "archives/mui-packages/mui-material/src/TablePagination/index.js"
    },
    {
      "name": "TablePaginationActions",
      "normalized": "table_pagination_actions",
      "source": "archives/mui-packages/mui-material/src/TablePaginationActions",
      "declared_in": "archives/mui-packages/mui-material/src/TablePaginationActions/index.js"
    },
    {
      "name": "TableRow",
      "normalized": "table_row",
      "source": "archives/mui-packages/mui-material/src/TableRow",
      "declared_in": "archives/mui-packages/mui-material/src/TableRow/index.js"
    },
    {
      "name": "TableSortLabel",
      "normalized": "table_sort_label",
      "source": "archives/mui-packages/mui-material/src/TableSortLabel",
      "declared_in": "archives/mui-packages/mui-material/src/TableSortLabel/index.js"
    },
    {
      "name": "Tabs",
      "normalized": "tabs",
      "source": "archives/mui-packages/mui-material/src/Tabs",
      "declared_in": "archives/mui-packages/mui-material/src/Tabs/index.js"
    },
    {
      "name": "TextareaAutosize",
      "normalized": "textarea_autosize",
      "source": "archives/mui-packages/mui-material/src/TextareaAutosize",
      "declared_in": "archives/mui-packages/mui-material/src/TextareaAutosize/index.js"
    },
    {
      "name": "THEME_ID",
      "normalized": "theme_id",
      "source": "archives/mui-packages/mui-material/src/identifier",
      "declared_in": "archives/mui-packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ThemeProvider",
      "normalized": "theme_provider",
      "source": "archives/mui-packages/mui-material/src/ThemeProvider",
      "declared_in": "archives/mui-packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ToggleButton",
      "normalized": "toggle_button",
      "source": "archives/mui-packages/mui-material/src/ToggleButton",
      "declared_in": "archives/mui-packages/mui-material/src/ToggleButton/index.js"
    },
    {
      "name": "ToggleButtonGroup",
      "normalized": "toggle_button_group",
      "source": "archives/mui-packages/mui-material/src/ToggleButtonGroup",
      "declared_in": "archives/mui-packages/mui-material/src/ToggleButtonGroup/index.js"
    },
    {
      "name": "Toolbar",
      "normalized": "toolbar",
      "source": "archives/mui-packages/mui-material/src/Toolbar",
      "declared_in": "archives/mui-packages/mui-material/src/Toolbar/index.js"
    },
    {
      "name": "Tooltip",
      "normalized": "tooltip",
      "source": "archives/mui-packages/mui-material/src/Tooltip",
      "declared_in": "archives/mui-packages/mui-material/src/Tooltip/index.js"
    },
    {
      "name": "Typography",
      "normalized": "typography",
      "source": "archives/mui-packages/mui-material/src/Typography",
      "declared_in": "archives/mui-packages/mui-material/src/Typography/index.js"
    },
    {
      "name": "Unstable_TrapFocus",
      "normalized": "unstable_trap_focus",
      "source": "archives/mui-packages/mui-material/src/Unstable_TrapFocus",
      "declared_in": "archives/mui-packages/mui-material/src/index.js"
    },
    {
      "name": "UseAutocomplete",
      "normalized": "use_autocomplete",
      "source": "archives/mui-packages/mui-material/src/useAutocomplete",
      "declared_in": "archives/mui-packages/mui-material/src/useAutocomplete/index.js"
    },
    {
      "name": "UseLazyRipple",
      "normalized": "use_lazy_ripple",
      "source": "archives/mui-packages/mui-material/src/useLazyRipple",
      "declared_in": "archives/mui-packages/mui-material/src/useLazyRipple/index.ts"
    },
    {
      "name": "UsePagination",
      "normalized": "use_pagination",
      "source": "archives/mui-packages/mui-material/src/usePagination",
      "declared_in": "archives/mui-packages/mui-material/src/usePagination/index.js"
    },
    {
      "name": "UseScrollTrigger",
      "normalized": "use_scroll_trigger",
      "source": "archives/mui-packages/mui-material/src/useScrollTrigger",
      "declared_in": "archives/mui-packages/mui-material/src/useScrollTrigger/index.js"
    },
    {
      "name": "Zoom",
      "normalized": "zoom",
      "source": "archives/mui-packages/mui-material/src/Zoom",
      "declared_in": "archives/mui-packages/mui-material/src/Zoom/index.js"
    }
  ],
  "missing_from_headless": [
    {
      "name": "Accordion",
      "normalized": "accordion",
      "source": "archives/mui-packages/mui-material/src/Accordion",
      "declared_in": "archives/mui-packages/mui-material/src/Accordion/index.js"
    },
    {
      "name": "AccordionActions",
      "normalized": "accordion_actions",
      "source": "archives/mui-packages/mui-material/src/AccordionActions",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionActions/index.js"
    },
    {
      "name": "AccordionDetails",
      "normalized": "accordion_details",
      "source": "archives/mui-packages/mui-material/src/AccordionDetails",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionDetails/index.js"
    },
    {
      "name": "AccordionSummary",
      "normalized": "accordion_summary",
      "source": "archives/mui-packages/mui-material/src/AccordionSummary",
      "declared_in": "archives/mui-packages/mui-material/src/AccordionSummary/index.js"
    },
    {
      "name": "Alert",
      "normalized": "alert",
      "source": "archives/mui-packages/mui-material/src/Alert",
      "declared_in": "archives/mui-packages/mui-material/src/Alert/index.js"
    },
    {
      "name": "AlertTitle",
      "normalized": "alert_title",
      "source": "archives/mui-packages/mui-material/src/AlertTitle",
      "declared_in": "archives/mui-packages/mui-material/src/AlertTitle/index.js"
    },
    {
      "name": "AppBar",
      "normalized": "app_bar",
      "source": "archives/mui-packages/mui-material/src/AppBar",
      "declared_in": "archives/mui-packages/mui-material/src/AppBar/index.js"
    },
    {
      "name": "Autocomplete",
      "normalized": "autocomplete",
      "source": "archives/mui-packages/mui-material/src/Autocomplete",
      "declared_in": "archives/mui-packages/mui-material/src/Autocomplete/index.js"
    },
    {
      "name": "Avatar",
      "normalized": "avatar",
      "source": "archives/mui-packages/mui-material/src/Avatar",
      "declared_in": "archives/mui-packages/mui-material/src/Avatar/index.js"
    },
    {
      "name": "AvatarGroup",
      "normalized": "avatar_group",
      "source": "archives/mui-packages/mui-material/src/AvatarGroup",
      "declared_in": "archives/mui-packages/mui-material/src/AvatarGroup/index.js"
    },
    {
      "name": "Backdrop",
      "normalized": "backdrop",
      "source": "archives/mui-packages/mui-material/src/Backdrop",
      "declared_in": "archives/mui-packages/mui-material/src/Backdrop/index.js"
    },
    {
      "name": "Badge",
      "normalized": "badge",
      "source": "archives/mui-packages/mui-material/src/Badge",
      "declared_in": "archives/mui-packages/mui-material/src/Badge/index.js"
    },
    {
      "name": "BottomNavigation",
      "normalized": "bottom_navigation",
      "source": "archives/mui-packages/mui-material/src/BottomNavigation",
      "declared_in": "archives/mui-packages/mui-material/src/BottomNavigation/index.js"
    },
    {
      "name": "BottomNavigationAction",
      "normalized": "bottom_navigation_action",
      "source": "archives/mui-packages/mui-material/src/BottomNavigationAction",
      "declared_in": "archives/mui-packages/mui-material/src/BottomNavigationAction/index.js"
    },
    {
      "name": "Box",
      "normalized": "box",
      "source": "archives/mui-packages/mui-material/src/Box",
      "declared_in": "archives/mui-packages/mui-material/src/Box/index.js"
    },
    {
      "name": "Breadcrumbs",
      "normalized": "breadcrumbs",
      "source": "archives/mui-packages/mui-material/src/Breadcrumbs",
      "declared_in": "archives/mui-packages/mui-material/src/Breadcrumbs/index.js"
    },
    {
      "name": "ButtonBase",
      "normalized": "button_base",
      "source": "archives/mui-packages/mui-material/src/ButtonBase",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonBase/index.js"
    },
    {
      "name": "ButtonGroup",
      "normalized": "button_group",
      "source": "archives/mui-packages/mui-material/src/ButtonGroup",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupButtonContext",
      "normalized": "button_group_button_context",
      "source": "archives/mui-packages/mui-material/src/ButtonGroupButtonContext",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "ButtonGroupContext",
      "normalized": "button_group_context",
      "source": "archives/mui-packages/mui-material/src/ButtonGroupContext",
      "declared_in": "archives/mui-packages/mui-material/src/ButtonGroup/index.js"
    },
    {
      "name": "Card",
      "normalized": "card",
      "source": "archives/mui-packages/mui-material/src/Card",
      "declared_in": "archives/mui-packages/mui-material/src/Card/index.js"
    },
    {
      "name": "CardActionArea",
      "normalized": "card_action_area",
      "source": "archives/mui-packages/mui-material/src/CardActionArea",
      "declared_in": "archives/mui-packages/mui-material/src/CardActionArea/index.js"
    },
    {
      "name": "CardActions",
      "normalized": "card_actions",
      "source": "archives/mui-packages/mui-material/src/CardActions",
      "declared_in": "archives/mui-packages/mui-material/src/CardActions/index.js"
    },
    {
      "name": "CardContent",
      "normalized": "card_content",
      "source": "archives/mui-packages/mui-material/src/CardContent",
      "declared_in": "archives/mui-packages/mui-material/src/CardContent/index.js"
    },
    {
      "name": "CardHeader",
      "normalized": "card_header",
      "source": "archives/mui-packages/mui-material/src/CardHeader",
      "declared_in": "archives/mui-packages/mui-material/src/CardHeader/index.js"
    },
    {
      "name": "CardMedia",
      "normalized": "card_media",
      "source": "archives/mui-packages/mui-material/src/CardMedia",
      "declared_in": "archives/mui-packages/mui-material/src/CardMedia/index.js"
    },
    {
      "name": "Checkbox",
      "normalized": "checkbox",
      "source": "archives/mui-packages/mui-material/src/Checkbox",
      "declared_in": "archives/mui-packages/mui-material/src/Checkbox/index.js"
    },
    {
      "name": "Chip",
      "normalized": "chip",
      "source": "archives/mui-packages/mui-material/src/Chip",
      "declared_in": "archives/mui-packages/mui-material/src/Chip/index.js"
    },
    {
      "name": "CircularProgress",
      "normalized": "circular_progress",
      "source": "archives/mui-packages/mui-material/src/CircularProgress",
      "declared_in": "archives/mui-packages/mui-material/src/CircularProgress/index.js"
    },
    {
      "name": "ClickAwayListener",
      "normalized": "click_away_listener",
      "source": "archives/mui-packages/mui-material/src/ClickAwayListener",
      "declared_in": "archives/mui-packages/mui-material/src/index.js"
    },
    {
      "name": "Collapse",
      "normalized": "collapse",
      "source": "archives/mui-packages/mui-material/src/Collapse",
      "declared_in": "archives/mui-packages/mui-material/src/Collapse/index.js"
    },
    {
      "name": "Container",
      "normalized": "container",
      "source": "archives/mui-packages/mui-material/src/Container",
      "declared_in": "archives/mui-packages/mui-material/src/Container/index.js"
    },
    {
      "name": "CssBaseline",
      "normalized": "css_baseline",
      "source": "archives/mui-packages/mui-material/src/CssBaseline",
      "declared_in": "archives/mui-packages/mui-material/src/CssBaseline/index.js"
    },
    {
      "name": "DefaultPropsProvider",
      "normalized": "default_props_provider",
      "source": "archives/mui-packages/mui-material/src/DefaultPropsProvider",
      "declared_in": "archives/mui-packages/mui-material/src/DefaultPropsProvider/index.ts"
    },
    {
      "name": "Dialog",
      "normalized": "dialog",
      "source": "archives/mui-packages/mui-material/src/Dialog",
      "declared_in": "archives/mui-packages/mui-material/src/Dialog/index.js"
    },
    {
      "name": "DialogActions",
      "normalized": "dialog_actions",
      "source": "archives/mui-packages/mui-material/src/DialogActions",
      "declared_in": "archives/mui-packages/mui-material/src/DialogActions/index.js"
    },
    {
      "name": "DialogContent",
      "normalized": "dialog_content",
      "source": "archives/mui-packages/mui-material/src/DialogContent",
      "declared_in": "archives/mui-packages/mui-material/src/DialogContent/index.js"
    },
    {
      "name": "DialogContentText",
      "normalized": "dialog_content_text",
      "source": "archives/mui-packages/mui-material/src/DialogContentText",
      "declared_in": "archives/mui-packages/mui-material/src/DialogContentText/index.js"
    },
    {
      "name": "DialogTitle",
      "normalized": "dialog_title",
      "source": "archives/mui-packages/mui-material/src/DialogTitle",
      "declared_in": "archives/mui-packages/mui-material/src/DialogTitle/index.js"
    },
    {
      "name": "Divider",
      "normalized": "divider",
      "source": "archives/mui-packages/mui-material/src/Divider",
      "declared_in": "archives/mui-packages/mui-material/src/Divider/index.js"
    },
    {
      "name": "Drawer",
      "normalized": "drawer",
      "source": "archives/mui-packages/mui-material/src/Drawer",
      "declared_in": "archives/mui-packages/mui-material/src/Drawer/index.js"
    },
    {
      "name": "Fab",
      "normalized": "fab",
      "source": "archives/mui-packages/mui-material/src/Fab",
      "declared_in": "archives/mui-packages/mui-material/src/Fab/index.js"
    },
    {
      "name": "Fade",
      "normalized": "fade",
      "source": "archives/mui-packages/mui-material/src/Fade",
      "declared_in": "archives/mui-packages/mui-material/src/Fade/index.js"
    },
    {
      "name": "FilledInput",
      "normalized": "filled_input",
      "source": "archives/mui-packages/mui-material/src/FilledInput",
      "declared_in": "archives/mui-packages/mui-material/src/FilledInput/index.js"
    },
    {
      "name": "FocusTrap",
      "normalized": "focus_trap",
      "source": "archives/mui-packages/mui-material/src/FocusTrap",
      "declared_in": "archives/mui-packages/mui-material/src/Unstable_TrapFocus/index.js"
    },
    {
      "name": "FormControl",
      "normalized": "form_control",
      "source": "archives/mui-packages/mui-material/src/FormControl",
      "declared_in": "archives/mui-packages/mui-material/src/FormControl/index.js"
    },
    {
      "name": "FormControlLabel",
      "normalized": "form_control_label",
      "source": "archives/mui-packages/mui-material/src/FormControlLabel",
      "declared_in": "archives/mui-packages/mui-material/src/FormControlLabel/index.js"
    },
    {
      "name": "FormGroup",
      "normalized": "form_group",
      "source": "archives/mui-packages/mui-material/src/FormGroup",
      "declared_in": "archives/mui-packages/mui-material/src/FormGroup/index.js"
    },
    {
      "name": "FormHelperText",
      "normalized": "form_helper_text",
      "source": "archives/mui-packages/mui-material/src/FormHelperText",
      "declared_in": "archives/mui-packages/mui-material/src/FormHelperText/index.js"
    },
    {
      "name": "FormLabel",
      "normalized": "form_label",
      "source": "archives/mui-packages/mui-material/src/FormLabel",
      "declared_in": "archives/mui-packages/mui-material/src/FormLabel/index.js"
    },
    {
      "name": "GlobalStyles",
      "normalized": "global_styles",
      "source": "archives/mui-packages/mui-material/src/GlobalStyles",
      "declared_in": "archives/mui-packages/mui-material/src/GlobalStyles/index.js"
    },
    {
      "name": "Grid",
      "normalized": "grid",
      "source": "archives/mui-packages/mui-material/src/Grid",
      "declared_in": "archives/mui-packages/mui-material/src/Grid/index.ts"
    },
    {
      "name": "Grow",
      "normalized": "grow",
      "source": "archives/mui-packages/mui-material/src/Grow",
      "declared_in": "archives/mui-packages/mui-material/src/Grow/index.js"
    },
    {
      "name": "Icon",
      "normalized": "icon",
      "source": "archives/mui-packages/mui-material/src/Icon",
      "declared_in": "archives/mui-packages/mui-material/src/Icon/index.js"
    },
    {
      "name": "IconButton",
      "normalized": "icon_button",
      "source": "archives/mui-packages/mui-material/src/IconButton",
      "declared_in": "archives/mui-packages/mui-material/src/IconButton/index.js"
    },
    {
      "name": "ImageList",
      "normalized": "image_list",
      "source": "archives/mui-packages/mui-material/src/ImageList",
      "declared_in": "archives/mui-packages/mui-material/src/ImageList/index.js"
    },
    {
      "name": "ImageListItem",
      "normalized": "image_list_item",
      "source": "archives/mui-packages/mui-material/src/ImageListItem",
      "declared_in": "archives/mui-packages/mui-material/src/ImageListItem/index.js"
    },
    {
      "name": "ImageListItemBar",
      "normalized": "image_list_item_bar",
      "source": "archives/mui-packages/mui-material/src/ImageListItemBar",
      "declared_in": "archives/mui-packages/mui-material/src/ImageListItemBar/index.js"
    },
    {
      "name": "InitColorSchemeScript",
      "normalized": "init_color_scheme_script",
      "source": "archives/mui-packages/mui-material/src/InitColorSchemeScript",
      "declared_in": "archives/mui-packages/mui-material/src/InitColorSchemeScript/index.ts"
    },
    {
      "name": "Input",
      "normalized": "input",
      "source": "archives/mui-packages/mui-material/src/Input",
      "declared_in": "archives/mui-packages/mui-material/src/Input/index.js"
    },
    {
      "name": "InputAdornment",
      "normalized": "input_adornment",
      "source": "archives/mui-packages/mui-material/src/InputAdornment",
      "declared_in": "archives/mui-packages/mui-material/src/InputAdornment/index.js"
    },
    {
      "name": "InputBase",
      "normalized": "input_base",
      "source": "archives/mui-packages/mui-material/src/InputBase",
      "declared_in": "archives/mui-packages/mui-material/src/InputBase/index.js"
    },
    {
      "name": "InputLabel",
      "normalized": "input_label",
      "source": "archives/mui-packages/mui-material/src/InputLabel",
      "declared_in": "archives/mui-packages/mui-material/src/InputLabel/index.js"
    },
    {
      "name": "LinearProgress",
      "normalized": "linear_progress",
      "source": "archives/mui-packages/mui-material/src/LinearProgress",
      "declared_in": "archives/mui-packages/mui-material/src/LinearProgress/index.js"
    },
    {
      "name": "Link",
      "normalized": "link",
      "source": "archives/mui-packages/mui-material/src/Link",
      "declared_in": "archives/mui-packages/mui-material/src/Link/index.js"
    },
    {
      "name": "List",
      "normalized": "list",
      "source": "archives/mui-packages/mui-material/src/List",
      "declared_in": "archives/mui-packages/mui-material/src/List/index.js"
    },
    {
      "name": "ListItem",
      "normalized": "list_item",
      "source": "archives/mui-packages/mui-material/src/ListItem",
      "declared_in": "archives/mui-packages/mui-material/src/ListItem/index.js"
    },
    {
      "name": "ListItemAvatar",
      "normalized": "list_item_avatar",
      "source": "archives/mui-packages/mui-material/src/ListItemAvatar",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemAvatar/index.js"
    },
    {
      "name": "ListItemButton",
      "normalized": "list_item_button",
      "source": "archives/mui-packages/mui-material/src/ListItemButton",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemButton/index.js"
    },
    {
      "name": "ListItemIcon",
      "normalized": "list_item_icon",
      "source": "archives/mui-packages/mui-material/src/ListItemIcon",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemIcon/index.js"
    },
    {
      "name": "ListItemSecondaryAction",
      "normalized": "list_item_secondary_action",
      "source": "archives/mui-packages/mui-material/src/ListItemSecondaryAction",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemSecondaryAction/index.js"
    },
    {
      "name": "ListItemText",
      "normalized": "list_item_text",
      "source": "archives/mui-packages/mui-material/src/ListItemText",
      "declared_in": "archives/mui-packages/mui-material/src/ListItemText/index.js"
    },
    {
      "name": "ListSubheader",
      "normalized": "list_subheader",
      "source": "archives/mui-packages/mui-material/src/ListSubheader",
      "declared_in": "archives/mui-packages/mui-material/src/ListSubheader/index.js"
    },
    {
      "name": "Menu",
      "normalized": "menu",
      "source": "archives/mui-packages/mui-material/src/Menu",
      "declared_in": "archives/mui-packages/mui-material/src/Menu/index.js"
    },
    {
      "name": "MenuItem",
      "normalized": "menu_item",
      "source": "archives/mui-packages/mui-material/src/MenuItem",
      "declared_in": "archives/mui-packages/mui-material/src/MenuItem/index.js"
    },
    {
      "name": "MenuList",
      "normalized": "menu_list",
      "source": "archives/mui-packages/mui-material/src/MenuList",
      "declared_in": "archives/mui-packages/mui-material/src/MenuList/index.js"
    },
    {
      "name": "MobileStepper",
      "normalized": "mobile_stepper",
      "source": "archives/mui-packages/mui-material/src/MobileStepper",
      "declared_in": "archives/mui-packages/mui-material/src/MobileStepper/index.js"
    },
    {
      "name": "Modal",
      "normalized": "modal",
      "source": "archives/mui-packages/mui-material/src/Modal",
      "declared_in": "archives/mui-packages/mui-material/src/Modal/index.js"
    },
    {
      "name": "NativeSelect",
      "normalized": "native_select",
      "source": "archives/mui-packages/mui-material/src/NativeSelect",
      "declared_in": "archives/mui-packages/mui-material/src/NativeSelect/index.js"
    },
    {
      "name": "NoSsr",
      "normalized": "no_ssr",
      "source": "archives/mui-packages/mui-material/src/NoSsr",
      "declared_in": "archives/mui-packages/mui-material/src/NoSsr/index.js"
    },
    {
      "name": "OutlinedInput",
      "normalized": "outlined_input",
      "source": "archives/mui-packages/mui-material/src/OutlinedInput",
      "declared_in": "archives/mui-packages/mui-material/src/OutlinedInput/index.js"
    },
    {
      "name": "Pagination",
      "normalized": "pagination",
      "source": "archives/mui-packages/mui-material/src/Pagination",
      "declared_in": "archives/mui-packages/mui-material/src/Pagination/index.js"
    },
    {
      "name": "PaginationItem",
      "normalized": "pagination_item",
      "source": "archives/mui-packages/mui-material/src/PaginationItem",
      "declared_in": "archives/mui-packages/mui-material/src/PaginationItem/index.js"
    },
    {
      "name": "Paper",
      "normalized": "paper",
      "source": "archives/mui-packages/mui-material/src/Paper",
      "declared_in": "archives/mui-packages/mui-material/src/Paper/index.js"
    },
    {
      "name": "PigmentContainer",
      "normalized": "pigment_container",
      "source": "archives/mui-packages/mui-material/src/PigmentContainer",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentContainer/index.ts"
    },
    {
      "name": "PigmentGrid",
      "normalized": "pigment_grid",
      "source": "archives/mui-packages/mui-material/src/PigmentGrid",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentGrid/index.ts"
    },
    {
      "name": "PigmentStack",
      "normalized": "pigment_stack",
      "source": "archives/mui-packages/mui-material/src/PigmentStack",
      "declared_in": "archives/mui-packages/mui-material/src/PigmentStack/index.ts"
    },
    {
      "name": "Popover",
      "normalized": "popover",
      "source": "archives/mui-packages/mui-material/src/Popover",
      "declared_in": "archives/mui-packages/mui-material/src/Popover/index.js"
    },
    {
      "name": "Popper",
      "normalized": "popper",
      "source": "archives/mui-packages/mui-material/src/Popper",
      "declared_in": "archives/mui-packages/mui-material/src/Popper/index.js"
    },
    {
      "name": "Portal",
      "normalized": "portal",
      "source": "archives/mui-packages/mui-material/src/Portal",
      "declared_in": "archives/mui-packages/mui-material/src/Portal/index.js"
    },
    {
      "name": "Radio",
      "normalized": "radio",
      "source": "archives/mui-packages/mui-material/src/Radio",
      "declared_in": "archives/mui-packages/mui-material/src/Radio/index.js"
    },
    {
      "name": "RadioGroup",
      "normalized": "radio_group",
      "source": "archives/mui-packages/mui-material/src/RadioGroup",
      "declared_in": "archives/mui-packages/mui-material/src/RadioGroup/index.js"
    },
    {
      "name": "Rating",
      "normalized": "rating",
      "source": "archives/mui-packages/mui-material/src/Rating",
      "declared_in": "archives/mui-packages/mui-material/src/Rating/index.js"
    },
    {
      "name": "ScopedCssBaseline",
      "normalized": "scoped_css_baseline",
      "source": "archives/mui-packages/mui-material/src/ScopedCssBaseline",
      "declared_in": "archives/mui-packages/mui-material/src/ScopedCssBaseline/index.js"
    },
    {
      "name": "Select",
      "normalized": "select",
      "source": "archives/mui-packages/mui-material/src/Select",
      "declared_in": "archives/mui-packages/mui-material/src/Select/index.js"
    },
    {
      "name": "Skeleton",
      "normalized": "skeleton",
      "source": "archives/mui-packages/mui-material/src/Skeleton",
      "declared_in": "archives/mui-packages/mui-material/src/Skeleton/index.js"
    },
    {
      "name": "Slide",
      "normalized": "slide",
      "source": "archives/mui-packages/mui-material/src/Slide",
      "declared_in": "archives/mui-packages/mui-material/src/Slide/index.js"
    },
    {
      "name": "Slider",
      "normalized": "slider",
      "source": "archives/mui-packages/mui-material/src/Slider",
      "declared_in": "archives/mui-packages/mui-material/src/Slider/index.js"
    },
    {
      "name": "Snackbar",
      "normalized": "snackbar",
      "source": "archives/mui-packages/mui-material/src/Snackbar",
      "declared_in": "archives/mui-packages/mui-material/src/Snackbar/index.js"
    },
    {
      "name": "SnackbarContent",
      "normalized": "snackbar_content",
      "source": "archives/mui-packages/mui-material/src/SnackbarContent",
      "declared_in": "archives/mui-packages/mui-material/src/SnackbarContent/index.js"
    },
    {
      "name": "SpeedDial",
      "normalized": "speed_dial",
      "source": "archives/mui-packages/mui-material/src/SpeedDial",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDial/index.js"
    },
    {
      "name": "SpeedDialAction",
      "normalized": "speed_dial_action",
      "source": "archives/mui-packages/mui-material/src/SpeedDialAction",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDialAction/index.js"
    },
    {
      "name": "SpeedDialIcon",
      "normalized": "speed_dial_icon",
      "source": "archives/mui-packages/mui-material/src/SpeedDialIcon",
      "declared_in": "archives/mui-packages/mui-material/src/SpeedDialIcon/index.js"
    },
    {
      "name": "Stack",
      "normalized": "stack",
      "source": "archives/mui-packages/mui-material/src/Stack",
      "declared_in": "archives/mui-packages/mui-material/src/Stack/index.js"
    },
    {
      "name": "Step",
      "normalized": "step",
      "source": "archives/mui-packages/mui-material/src/Step",
      "declared_in": "archives/mui-packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepButton",
      "normalized": "step_button",
      "source": "archives/mui-packages/mui-material/src/StepButton",
      "declared_in": "archives/mui-packages/mui-material/src/StepButton/index.js"
    },
    {
      "name": "StepConnector",
      "normalized": "step_connector",
      "source": "archives/mui-packages/mui-material/src/StepConnector",
      "declared_in": "archives/mui-packages/mui-material/src/StepConnector/index.js"
    },
    {
      "name": "StepContent",
      "normalized": "step_content",
      "source": "archives/mui-packages/mui-material/src/StepContent",
      "declared_in": "archives/mui-packages/mui-material/src/StepContent/index.js"
    },
    {
      "name": "StepContext",
      "normalized": "step_context",
      "source": "archives/mui-packages/mui-material/src/StepContext",
      "declared_in": "archives/mui-packages/mui-material/src/Step/index.js"
    },
    {
      "name": "StepIcon",
      "normalized": "step_icon",
      "source": "archives/mui-packages/mui-material/src/StepIcon",
      "declared_in": "archives/mui-packages/mui-material/src/StepIcon/index.js"
    },
    {
      "name": "StepLabel",
      "normalized": "step_label",
      "source": "archives/mui-packages/mui-material/src/StepLabel",
      "declared_in": "archives/mui-packages/mui-material/src/StepLabel/index.js"
    },
    {
      "name": "Stepper",
      "normalized": "stepper",
      "source": "archives/mui-packages/mui-material/src/Stepper",
      "declared_in": "archives/mui-packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "StepperContext",
      "normalized": "stepper_context",
      "source": "archives/mui-packages/mui-material/src/StepperContext",
      "declared_in": "archives/mui-packages/mui-material/src/Stepper/index.js"
    },
    {
      "name": "SvgIcon",
      "normalized": "svg_icon",
      "source": "archives/mui-packages/mui-material/src/SvgIcon",
      "declared_in": "archives/mui-packages/mui-material/src/SvgIcon/index.js"
    },
    {
      "name": "SwipeableDrawer",
      "normalized": "swipeable_drawer",
      "source": "archives/mui-packages/mui-material/src/SwipeableDrawer",
      "declared_in": "archives/mui-packages/mui-material/src/SwipeableDrawer/index.js"
    },
    {
      "name": "Switch",
      "normalized": "switch",
      "source": "archives/mui-packages/mui-material/src/Switch",
      "declared_in": "archives/mui-packages/mui-material/src/Switch/index.js"
    },
    {
      "name": "Tab",
      "normalized": "tab",
      "source": "archives/mui-packages/mui-material/src/Tab",
      "declared_in": "archives/mui-packages/mui-material/src/Tab/index.js"
    },
    {
      "name": "TabScrollButton",
      "normalized": "tab_scroll_button",
      "source": "archives/mui-packages/mui-material/src/TabScrollButton",
      "declared_in": "archives/mui-packages/mui-material/src/TabScrollButton/index.js"
    },
    {
      "name": "Table",
      "normalized": "table",
      "source": "archives/mui-packages/mui-material/src/Table",
      "declared_in": "archives/mui-packages/mui-material/src/Table/index.js"
    },
    {
      "name": "TableBody",
      "normalized": "table_body",
      "source": "archives/mui-packages/mui-material/src/TableBody",
      "declared_in": "archives/mui-packages/mui-material/src/TableBody/index.js"
    },
    {
      "name": "TableCell",
      "normalized": "table_cell",
      "source": "archives/mui-packages/mui-material/src/TableCell",
      "declared_in": "archives/mui-packages/mui-material/src/TableCell/index.js"
    },
    {
      "name": "TableContainer",
      "normalized": "table_container",
      "source": "archives/mui-packages/mui-material/src/TableContainer",
      "declared_in": "archives/mui-packages/mui-material/src/TableContainer/index.js"
    },
    {
      "name": "TableFooter",
      "normalized": "table_footer",
      "source": "archives/mui-packages/mui-material/src/TableFooter",
      "declared_in": "archives/mui-packages/mui-material/src/TableFooter/index.js"
    },
    {
      "name": "TableHead",
      "normalized": "table_head",
      "source": "archives/mui-packages/mui-material/src/TableHead",
      "declared_in": "archives/mui-packages/mui-material/src/TableHead/index.js"
    },
    {
      "name": "TablePagination",
      "normalized": "table_pagination",
      "source": "archives/mui-packages/mui-material/src/TablePagination",
      "declared_in": "archives/mui-packages/mui-material/src/TablePagination/index.js"
    },
    {
      "name": "TablePaginationActions",
      "normalized": "table_pagination_actions",
      "source": "archives/mui-packages/mui-material/src/TablePaginationActions",
      "declared_in": "archives/mui-packages/mui-material/src/TablePaginationActions/index.js"
    },
    {
      "name": "TableRow",
      "normalized": "table_row",
      "source": "archives/mui-packages/mui-material/src/TableRow",
      "declared_in": "archives/mui-packages/mui-material/src/TableRow/index.js"
    },
    {
      "name": "TableSortLabel",
      "normalized": "table_sort_label",
      "source": "archives/mui-packages/mui-material/src/TableSortLabel",
      "declared_in": "archives/mui-packages/mui-material/src/TableSortLabel/index.js"
    },
    {
      "name": "Tabs",
      "normalized": "tabs",
      "source": "archives/mui-packages/mui-material/src/Tabs",
      "declared_in": "archives/mui-packages/mui-material/src/Tabs/index.js"
    },
    {
      "name": "TextField",
      "normalized": "text_field",
      "source": "archives/mui-packages/mui-material/src/TextField",
      "declared_in": "archives/mui-packages/mui-material/src/TextField/index.js"
    },
    {
      "name": "TextareaAutosize",
      "normalized": "textarea_autosize",
      "source": "archives/mui-packages/mui-material/src/TextareaAutosize",
      "declared_in": "archives/mui-packages/mui-material/src/TextareaAutosize/index.js"
    },
    {
      "name": "THEME_ID",
      "normalized": "theme_id",
      "source": "archives/mui-packages/mui-material/src/identifier",
      "declared_in": "archives/mui-packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ThemeProvider",
      "normalized": "theme_provider",
      "source": "archives/mui-packages/mui-material/src/ThemeProvider",
      "declared_in": "archives/mui-packages/mui-material/src/styles/index.js"
    },
    {
      "name": "ToggleButton",
      "normalized": "toggle_button",
      "source": "archives/mui-packages/mui-material/src/ToggleButton",
      "declared_in": "archives/mui-packages/mui-material/src/ToggleButton/index.js"
    },
    {
      "name": "ToggleButtonGroup",
      "normalized": "toggle_button_group",
      "source": "archives/mui-packages/mui-material/src/ToggleButtonGroup",
      "declared_in": "archives/mui-packages/mui-material/src/ToggleButtonGroup/index.js"
    },
    {
      "name": "Toolbar",
      "normalized": "toolbar",
      "source": "archives/mui-packages/mui-material/src/Toolbar",
      "declared_in": "archives/mui-packages/mui-material/src/Toolbar/index.js"
    },
    {
      "name": "Tooltip",
      "normalized": "tooltip",
      "source": "archives/mui-packages/mui-material/src/Tooltip",
      "declared_in": "archives/mui-packages/mui-material/src/Tooltip/index.js"
    },
    {
      "name": "Typography",
      "normalized": "typography",
      "source": "archives/mui-packages/mui-material/src/Typography",
      "declared_in": "archives/mui-packages/mui-material/src/Typography/index.js"
    },
    {
      "name": "Unstable_TrapFocus",
      "normalized": "unstable_trap_focus",
      "source": "archives/mui-packages/mui-material/src/Unstable_TrapFocus",
      "declared_in": "archives/mui-packages/mui-material/src/index.js"
    },
    {
      "name": "UseAutocomplete",
      "normalized": "use_autocomplete",
      "source": "archives/mui-packages/mui-material/src/useAutocomplete",
      "declared_in": "archives/mui-packages/mui-material/src/useAutocomplete/index.js"
    },
    {
      "name": "UseLazyRipple",
      "normalized": "use_lazy_ripple",
      "source": "archives/mui-packages/mui-material/src/useLazyRipple",
      "declared_in": "archives/mui-packages/mui-material/src/useLazyRipple/index.ts"
    },
    {
      "name": "UsePagination",
      "normalized": "use_pagination",
      "source": "archives/mui-packages/mui-material/src/usePagination",
      "declared_in": "archives/mui-packages/mui-material/src/usePagination/index.js"
    },
    {
      "name": "UseScrollTrigger",
      "normalized": "use_scroll_trigger",
      "source": "archives/mui-packages/mui-material/src/useScrollTrigger",
      "declared_in": "archives/mui-packages/mui-material/src/useScrollTrigger/index.js"
    },
    {
      "name": "Zoom",
      "normalized": "zoom",
      "source": "archives/mui-packages/mui-material/src/Zoom",
      "declared_in": "archives/mui-packages/mui-material/src/Zoom/index.js"
    }
  ],
  "extra_in_material": [],
  "extra_in_headless": [
    "aria"
  ]
}
```
