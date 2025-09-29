//! Responsive state for Masonry and Standard image lists.
//!
//! Image lists appear deceptively simple but quickly grow complex once masonry
//! layouts, row heights and accessibility hooks are accounted for.  Centralising
//! this logic keeps SSR and CSR adapters deterministic while also documenting
//! the scalability trade-offs (e.g. using integers for column counts rather than
//! precomputed CSS strings).

use crate::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

/// Visual variant of the image list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageListVariant {
    /// Standard grid layout.
    Standard,
    /// Masonry layout with variable row spans.
    Masonry,
}

impl ImageListVariant {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Masonry => "masonry",
        }
    }
}

/// Default ARIA roles for the image list container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageListRole {
    /// Expose the list as a semantic list of images.
    List,
    /// Presentation role when the image list is purely decorative.
    Presentation,
}

impl ImageListRole {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Presentation => "presentation",
        }
    }
}

impl Default for ImageListRole {
    fn default() -> Self {
        Self::List
    }
}

/// Responsive tokens describing how the image list should render.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageListTokens {
    /// Number of columns at each breakpoint.
    pub columns: ResponsiveValue<u16>,
    /// Gap token between items.
    pub gap: ResponsiveValue<String>,
    /// Target row height for masonry layouts (when applicable).
    pub row_height: ResponsiveValue<u16>,
}

impl ImageListTokens {
    /// Build a token set with uniform gap and row height but responsive columns.
    pub fn uniform(columns: ResponsiveValue<u16>, gap: impl Into<String>, row_height: u16) -> Self {
        Self {
            columns,
            gap: ResponsiveValue::from(gap.into()),
            row_height: ResponsiveValue::from(row_height),
        }
    }
}

/// State shared across adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageListState {
    tokens: ImageListTokens,
    breakpoints: BreakpointConfig,
    role: ImageListRole,
    variant: ImageListVariant,
}

impl ImageListState {
    /// Create a new image list state.
    pub fn new(tokens: ImageListTokens, breakpoints: BreakpointConfig) -> Self {
        Self {
            tokens,
            breakpoints,
            role: ImageListRole::default(),
            variant: ImageListVariant::Standard,
        }
    }

    /// Configure the variant.
    pub fn variant(mut self, variant: ImageListVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Override the ARIA role.
    pub fn with_role(mut self, role: ImageListRole) -> Self {
        self.role = role;
        self
    }

    #[inline]
    pub fn breakpoints(&self) -> &BreakpointConfig {
        &self.breakpoints
    }

    #[inline]
    pub fn evaluate(&self, viewport_width: u32) -> ImageListEvaluation<'_> {
        let breakpoint = self.breakpoints.active_at(viewport_width);
        self.evaluate_for(breakpoint)
    }

    #[inline]
    pub fn evaluate_for(&self, breakpoint: Breakpoint) -> ImageListEvaluation<'_> {
        ImageListEvaluation {
            breakpoint,
            columns: *self.tokens.columns.value_for(breakpoint),
            gap: self.tokens.gap.value_for(breakpoint),
            row_height: *self.tokens.row_height.value_for(breakpoint),
            role: self.role,
            variant: self.variant,
        }
    }

    #[inline]
    pub fn attributes(&self) -> ImageListAttributes<'_> {
        ImageListAttributes {
            state: self,
            id: None,
            class: None,
        }
    }
}

/// Result of evaluating the image list tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageListEvaluation<'a> {
    pub breakpoint: Breakpoint,
    pub columns: u16,
    pub gap: &'a String,
    pub row_height: u16,
    pub role: ImageListRole,
    pub variant: ImageListVariant,
}

/// Attribute builder mirroring other layout primitives.
#[derive(Debug, Clone)]
pub struct ImageListAttributes<'a> {
    state: &'a ImageListState,
    id: Option<&'a str>,
    class: Option<&'a str>,
}

impl<'a> ImageListAttributes<'a> {
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    #[inline]
    pub fn class(mut self, value: &'a str) -> Self {
        self.class = Some(value);
        self
    }

    #[inline]
    pub fn role(&self) -> (&'static str, &'static str) {
        ("role", self.state.role.as_str())
    }

    #[inline]
    pub fn data_variant(&self) -> (&'static str, &'static str) {
        ("data-variant", self.state.variant.as_str())
    }

    #[inline]
    pub fn data_breakpoint(&self, viewport_width: u32) -> (&'static str, &'static str) {
        let breakpoint = self.state.breakpoints.active_at(viewport_width);
        ("data-breakpoint", breakpoint.as_token())
    }

    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    #[inline]
    pub fn class_attr(&self) -> Option<(&'static str, &str)> {
        self.class.map(|value| ("class", value))
    }
}
