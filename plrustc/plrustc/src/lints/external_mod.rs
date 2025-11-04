use rustc_ast as ast;
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};

declare_plrust_lint!(
    pub(crate) PLRUST_EXTERNAL_MOD,
    "Disallow use of `mod blah;`",
);

rustc_lint_defs::declare_lint_pass!(PlrustExternalMod => [PLRUST_EXTERNAL_MOD]);

impl EarlyLintPass for PlrustExternalMod {
    fn check_item(&mut self, cx: &EarlyContext, item: &ast::Item) {
        match &item.kind {
            ast::ItemKind::Mod(_, ast::ModKind::Unloaded)
            | ast::ItemKind::Mod(_, ast::ModKind::Loaded(_, ast::Inline::No, _, _)) => {
                cx.lint(PLRUST_EXTERNAL_MOD, |diag| {
                    diag.primary_message("Use of external modules is forbidden in PL/Rust");
                    diag.span(item.span);
                });
            }
            _ => {}
        }
    }
}
