use std::fmt::{Debug, Display, Formatter};
use syn::spanned::Spanned;

/// Simple Diagnostic trait for use in proc_macro and for testing.
pub trait Diagnostic {
    /// Emit a warning.
    fn emit_warning<S: Spanned, T: Display>(&mut self, span: S, message: T);

    /// Emit an error.
    fn emit_error<S: Spanned, T: Display>(&mut self, span: S, message: T);

    /// This should abort (i.e. panic) if an error did occur.
    ///
    /// But maybe not in a test case.
    fn should_abort_if_dirty(&mut self);
}

/// Simulated error diagnostics.
///
/// After the first abort no more messages are recorded.
///
/// Use the `assert*` function to verify the expected result.
pub struct Simulated {
    aborted: bool,
    warnings: Vec<String>,
    errors: Vec<String>,
}

impl Simulated {
    /// Create a new instance.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            aborted: false,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Run a function and return a failure if any warnings/errors occurs.
    #[inline]
    pub(crate) fn run<F, T>(f: F) -> Result<T, Self>
    where
        F: FnOnce(&mut Simulated) -> T,
    {
        let mut simulated = Self::new();
        let result = f(&mut simulated);
        if simulated.warnings.is_empty() && simulated.errors.is_empty() {
            Ok(result)
        } else {
            Err(simulated)
        }
    }

    /// Assert that there are no messages.
    pub fn assert_empty(&self) {
        self.assert(&[], &[]);
    }

    /// Assert that there are the specified warnings and no errors.
    pub fn assert_warnings(&self, warnings: &[&str]) {
        self.assert(warnings, &[]);
    }

    /// Assert that there are the specified errors and no warnings.
    pub fn assert_errors(&self, errors: &[&str]) {
        self.assert(&[], errors);
    }

    /// Assert that there are the specified warnings and errors.
    pub fn assert(&self, warnings: &[&str], errors: &[&str]) {
        assert_eq!(self.warnings, warnings);
        assert_eq!(self.errors, errors);
    }
}

impl Debug for Simulated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.warnings.iter().chain(self.errors.iter()))
            .finish()
    }
}

impl Diagnostic for Simulated {
    fn emit_warning<S: Spanned, T: Display>(&mut self, span: S, message: T) {
        if !self.aborted {
            self.warnings.push(format!("{:?}: {message}", span.span()));
        }
    }

    fn emit_error<S: Spanned, T: Display>(&mut self, span: S, message: T) {
        if !self.aborted {
            self.errors.push(format!("{:?}: {message}", span.span()));
        }
    }

    fn should_abort_if_dirty(&mut self) {
        if !self.errors.is_empty() {
            self.aborted = true;
        }
    }
}

/// Diagnostics with the crate `proc_macro_error`.
pub struct ProcMacroError;

impl Diagnostic for ProcMacroError {
    fn emit_warning<S: Spanned, T: Display>(&mut self, span: S, message: T) {
        proc_macro_error::emit_warning!(span.span(), message);
    }

    fn emit_error<S: Spanned, T: Display>(&mut self, span: S, message: T) {
        proc_macro_error::emit_error!(span.span(), message);
    }

    fn should_abort_if_dirty(&mut self) {
        proc_macro_error::abort_if_dirty();
    }
}
