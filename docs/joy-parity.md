# Joy UI Rust Parity Audit

This document enumerates the Joy UI surface exported from
`packages/mui-joy/src/index.ts` and maps every widget to the equivalent (or
planned) Rust support.  Interactive components — the ones that need a state
machine rather than purely presentational markup — are called out explicitly so
we can close gaps in `mui-headless` and keep downstream Joy adapters fully
accessible.

| Component | Interactive | Rust status | Notes |
| --- | --- | --- | --- |
| Accordion | ✅ | **New:** `rustic_ui_headless::accordion` | Disclosure widget coordinating expanded/collapsed panels. |
| AccordionDetails | ❌ | Structural | Presentational container nested within accordion items. |
| AccordionGroup | ✅ | **New:** `rustic_ui_headless::accordion` | Group manager that enforces single/multi expansion policies. |
| AccordionSummary | ✅ | **New:** `rustic_ui_headless::accordion` | Trigger surface wired into the accordion change stream. |
| Alert | ⚠️ | Not yet modelled | Mostly presentational; dismissal will reuse snackbar queueing later. |
| AspectRatio | ❌ | Available (`crates/rustic-ui-joy::aspect_ratio`) | Layout helper without interactivity. |
| Autocomplete | ✅ | **New:** `rustic_ui_headless::autocomplete` | Hybrid text input + listbox that reuses select patterns. |
| AutocompleteListbox | ✅ | **New:** `rustic_ui_headless::autocomplete` | Popover list that mirrors Joy’s listbox styling. |
| AutocompleteOption | ✅ | **New:** `rustic_ui_headless::autocomplete` | Option level helpers for automation IDs and ARIA wiring. |
| Avatar | ❌ | Pending Joy port | Static display; no state machine needed. |
| AvatarGroup | ❌ | Pending Joy port | Layout driven aggregation of avatars. |
| Badge | ❌ | Pending Joy port | Static counter badge. |
| Box | ❌ | Pending Joy port | Generic layout primitive. |
| Breadcrumbs | ❌ | Pending Joy port | Navigation aid without local state. |
| Button | ✅ | Available (`rustic_ui_headless::button`) | Button state is already centralised and consumed by Joy. |
| ButtonGroup | ✅ | **New:** `rustic_ui_headless::toggle_button_group` | Shared toggle orchestration for grouped buttons. |
| Card | ❌ | Available (`crates/rustic-ui-joy::card`) | Presentational Joy component. |
| CardActions | ❌ | Pending Joy port | Layout helper. |
| CardContent | ❌ | Pending Joy port | Layout helper. |
| CardCover | ❌ | Pending Joy port | Layout helper. |
| CardOverflow | ❌ | Pending Joy port | Layout helper. |
| Checkbox | ✅ | Available (`rustic_ui_headless::checkbox`) | Already wired through Material + Joy. |
| Chip | ✅ | Available (`rustic_ui_headless::chip`) | Hover/delete automation already documented. |
| ChipDelete | ✅ | Available (`rustic_ui_headless::chip`) | Uses chip trailing action state. |
| CircularProgress | ❌ | Pending Joy port | Visual only. |
| Container | ❌ | Pending Joy port | Layout wrapper. |
| CssBaseline | ❌ | Pending Joy port | Global CSS reset. |
| DialogActions | ❌ | Pending Joy port | Static region in Joy dialogs. |
| DialogContent | ❌ | Pending Joy port | Static region in Joy dialogs. |
| DialogTitle | ❌ | Pending Joy port | Static region in Joy dialogs. |
| Divider | ❌ | Pending Joy port | Visual only. |
| Drawer | ✅ | Available (`rustic_ui_headless::drawer`) | Shares dialog style modal logic. |
| Dropdown | ✅ | Available (`rustic_ui_headless::menu`) | Wraps menu/listbox behavior. |
| FormControl | ❌ | Pending Joy port | Structural helper. |
| FormHelperText | ❌ | Pending Joy port | Static text. |
| FormLabel | ❌ | Pending Joy port | Static label. |
| Grid | ❌ | Pending Joy port | Layout system. |
| IconButton | ✅ | Available (`rustic_ui_headless::button`) | Button variant. |
| Input | ✅ | Available (`rustic_ui_headless::text_field`) | Shares the text field state machine. |
| LinearProgress | ❌ | Pending Joy port | Visual only. |
| Link | ✅ | Pending Joy port | Minimal interactivity, standard anchor semantics. |
| List | ✅ | Available (`rustic_ui_headless::list`) | Keyboard navigation + typeahead. |
| ListDivider | ❌ | Pending Joy port | Visual only. |
| ListItem | ✅ | Available (`rustic_ui_headless::list`) | Delegates to list navigation logic. |
| ListItemButton | ✅ | Available (`rustic_ui_headless::list`) | Delegates to list navigation logic. |
| ListItemContent | ❌ | Pending Joy port | Static container. |
| ListItemDecorator | ❌ | Pending Joy port | Static container. |
| ListSubheader | ❌ | Pending Joy port | Static container. |
| Menu | ✅ | Available (`rustic_ui_headless::menu`) | Driven by menu state machine. |
| MenuButton | ✅ | Available (`rustic_ui_headless::menu`) | Uses the menu trigger helpers. |
| MenuItem | ✅ | Available (`rustic_ui_headless::menu`) | Option level helpers. |
| MenuList | ✅ | Available (`rustic_ui_headless::menu`) | Popover container. |
| Modal | ✅ | Available (`rustic_ui_headless::dialog`) | Shares dialog state machine for overlays. |
| ModalClose | ✅ | Available (`rustic_ui_headless::dialog`) | Uses dialog close intents. |
| ModalDialog | ✅ | Available (`rustic_ui_headless::dialog`) | Uses dialog positioning + focus trap. |
| ModalOverflow | ❌ | Pending Joy port | Presentational. |
| Option | ✅ | Available (`rustic_ui_headless::select`) | Headless select option state. |
| Radio | ✅ | Available (`rustic_ui_headless::radio`) | State machine shared with Material. |
| RadioGroup | ✅ | Available (`rustic_ui_headless::radio`) | Group level orchestrator. |
| ScopedCssBaseline | ❌ | Pending Joy port | Global CSS helper. |
| Select | ✅ | Available (`rustic_ui_headless::select`) | Comprehensive listbox state. |
| Sheet | ❌ | Pending Joy port | Layout container. |
| Skeleton | ❌ | Pending Joy port | Visual only. |
| Slider | ✅ | **New:** `rustic_ui_headless::slider` | Value tracking, keyboard/pointer handling. |
| Snackbar | ✅ | **New:** `rustic_ui_headless::snackbar` | Timed queue mirroring Material behavior. |
| Stepper | ✅ | **New:** `rustic_ui_headless::stepper` | Linear/non-linear progress management. |
| Step | ✅ | **New:** `rustic_ui_headless::stepper` | Per-step completion bookkeeping. |
| StepButton | ✅ | **New:** `rustic_ui_headless::stepper` | Focusable trigger for navigation. |
| StepIndicator | ✅ | **New:** `rustic_ui_headless::stepper` | Derives status from stepper state. |
| Stack | ❌ | Pending Joy port | Layout primitive. |
| SvgIcon | ❌ | Pending Joy port | Visual only. |
| Switch | ✅ | Available (`rustic_ui_headless::switch`) | Toggle machine already present. |
| Tab | ✅ | Available (`rustic_ui_headless::tab`) | Tab level helpers. |
| Table | ⚠️ | Material driven (`rustic_ui_material::table`) | Mostly structural; Joy equivalent TBD. |
| TabList | ✅ | Available (`rustic_ui_headless::tabs`) | Tablist orchestration. |
| TabPanel | ✅ | Available (`rustic_ui_headless::tab_panel`) | Panel mapping. |
| Tabs | ✅ | Available (`rustic_ui_headless::tabs`) | High level orchestrator. |
| Textarea | ✅ | Available (`rustic_ui_headless::text_field`) | Shares text field state machine. |
| TextField | ✅ | Available (`rustic_ui_headless::text_field`) | Already implemented. |
| ToggleButtonGroup | ✅ | **New:** `rustic_ui_headless::toggle_button_group` | Exclusive/multi selection toggles. |
| Tooltip | ✅ | Available (`rustic_ui_headless::tooltip`) | Hover/focus timing machine. |
| Typography | ❌ | Pending Joy port | Static text styles. |

## Next steps

The newly introduced headless modules (`accordion`, `autocomplete`, `slider`,
`snackbar`, `stepper`, `toggle_button_group`) unlock Joy specific wrappers in
`crates/rustic-ui-joy`.  Remaining non-interactive components can be brought across
iteratively without headless changes, while future interactive widgets (e.g.
Alert dismissal, advanced tables) should follow the same headless-first
pattern.
