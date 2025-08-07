// Modified and extended code from
// https://github.com/starkware-libs/cairo/blob/932767340c4b8a762140bf9eba305f437587ac1b/crates/cairo-lang-parser/src/printer.rs.

use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::ast::SyntaxFile;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::green::GreenNodeDetails;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax_codegen::cairo_spec::get_spec;
use cairo_lang_syntax_codegen::spec::{Member, Node, NodeKind};
use cairo_lang_utils::Intern;
use colored::{ColoredString, Colorize};
use itertools::zip_eq;

pub fn file_syntax_tree(db: &dyn SyntaxGroup, syntax_root: &SyntaxNode) -> String {
    let mut printer = Printer::new(db, true, false);
    printer.print_tree("root", syntax_root, "", true);
    printer.result
}

pub fn syntax_tree_branch_above_leaf(
    db: &dyn SyntaxGroup,
    syntax_tree_leaf: &SyntaxNode,
) -> String {
    let mut syntax_tree_branch: Vec<_> = syntax_tree_leaf.ancestors_with_self(db).collect();
    syntax_tree_branch.reverse();

    let mut printer = Printer::new(db, true, false);

    let mut field_description = "root".to_string();

    for window in syntax_tree_branch.windows(2) {
        let [this, child] = window else { break };
        let green_node = this.green_node(db);
        match &green_node.details {
            GreenNodeDetails::Node { .. } => {
                printer.print_only_internal_node(this, &field_description, green_node.kind);
            }
            GreenNodeDetails::Token(_) => unreachable!("Only the last node can be the token"),
        }

        let child_green = child.green_node(db).clone().intern(db);
        let mut child_num = None;
        match &green_node.details {
            GreenNodeDetails::Node { children, .. } => {
                for (i, c) in children.iter().enumerate() {
                    if *c == child_green {
                        child_num = Some(i);
                        break;
                    }
                }
            }
            GreenNodeDetails::Token(_) => unreachable!("Only the last node can be the token"),
        };
        let child_num = child_num.unwrap();

        let node_kind = printer.get_node_kind(green_node.kind.to_string());

        match node_kind {
            NodeKind::Struct { members: expected_children }
            | NodeKind::Terminal { members: expected_children, .. } => {
                field_description = expected_children[child_num].name.clone();
            }
            NodeKind::List { .. } => {
                field_description = format!("child #{child_num}");
            }
            NodeKind::SeparatedList { .. } => {
                let description = if child_num % 2 == 0 { "item" } else { "separator" };
                field_description = format!("{description} #{}", child_num / 2);
            }
            _ => unreachable!("This should never happen"),
        }
    }

    let last = syntax_tree_branch.last().unwrap();
    let green_node = last.green_node(db);
    match &green_node.details {
        GreenNodeDetails::Token(text) => {
            printer.print_token_node(&field_description, "", "", text, green_node.kind)
        }
        GreenNodeDetails::Node { children, .. } => {
            assert_eq!(green_node.kind, SyntaxKind::SyntaxFile);
            printer.print_only_internal_node(last, &field_description, green_node.kind);

            let eof_green = children[SyntaxFile::INDEX_EOF].long(db).children()[1].long(db);
            match &eof_green.details {
                GreenNodeDetails::Token(text) => {
                    printer.print_token_node("eof", "", "", text, eof_green.kind)
                }
                GreenNodeDetails::Node { .. } => unreachable!(),
            }
        }
    }

    printer.result
}

struct Printer<'a> {
    db: &'a dyn SyntaxGroup,
    spec: Vec<Node>,
    print_colors: bool,
    print_trivia: bool,
    result: String,
}
impl<'a> Printer<'a> {
    fn new(db: &'a dyn SyntaxGroup, print_colors: bool, print_trivia: bool) -> Self {
        Self { db, spec: get_spec(), print_colors, print_trivia, result: String::new() }
    }

    /// `under_top_level`: whether we are in a subtree of the top-level kind.
    fn print_tree(
        &mut self,
        field_description: &str,
        syntax_node: &SyntaxNode,
        indent: &str,
        is_last: bool,
    ) {
        let extra_head_indent = if is_last { "└── " } else { "├── " };
        let green_node = syntax_node.green_node(self.db);
        match &green_node.details {
            GreenNodeDetails::Token(text) => self.print_token_node(
                field_description,
                indent,
                extra_head_indent,
                text,
                green_node.kind,
            ),
            GreenNodeDetails::Node { .. } => {
                self.print_internal_node(
                    field_description,
                    indent,
                    extra_head_indent,
                    is_last,
                    syntax_node,
                    green_node.kind,
                );
            }
        }
    }

    fn print_token_node(
        &mut self,
        field_description: &str,
        indent: &str,
        extra_head_indent: &str,
        text: &str,
        kind: SyntaxKind,
    ) {
        let text = if kind == SyntaxKind::TokenMissing {
            format!("{}: {}", self.blue(field_description.into()), self.red("Missing".into()))
        } else {
            let token_text = match kind {
                SyntaxKind::TokenWhitespace
                | SyntaxKind::TokenNewline
                | SyntaxKind::TokenEndOfFile => ".".to_string(),
                _ => format!(": '{}'", self.green(self.bold(text.into()))),
            };
            format!("{} (kind: {:?}){token_text}", self.blue(field_description.into()), kind)
        };
        self.result.push_str(format!("{indent}{extra_head_indent}{text}\n").as_str());
    }

    fn print_only_internal_node(
        &mut self,
        syntax_node: &SyntaxNode,
        field_description: &str,
        kind: SyntaxKind,
    ) {
        let extra_info = if is_missing_kind(kind) {
            format!(": {}", self.red("Missing".into()))
        } else {
            format!(" (kind: {kind:?})")
        };

        let children = syntax_node.get_children(self.db);
        let num_children = children.len();
        let suffix = if num_children == 0 {
            self.bright_purple(" []".into()).to_string()
        } else {
            String::new()
        };

        self.result.push_str(
            format!("{}{extra_info}{suffix}\n", self.cyan(field_description.into())).as_str(),
        );
    }

    /// `under_top_level`: whether we are in a subtree of the top-level kind.
    #[allow(clippy::too_many_arguments)]
    fn print_internal_node(
        &mut self,
        field_description: &str,
        indent: &str,
        extra_head_indent: &str,
        is_last: bool,
        syntax_node: &SyntaxNode,
        kind: SyntaxKind,
    ) {
        if !self.print_trivia
            && let Some(token_node) = syntax_node.get_terminal_token(self.db)
        {
            self.print_tree(field_description, &token_node, indent, is_last);
            return;
        }

        let extra_info = if is_missing_kind(kind) {
            format!(": {}", self.red("Missing".into()))
        } else {
            format!(" (kind: {kind:?})")
        };

        let children = syntax_node.get_children(self.db);
        let num_children = children.len();
        let suffix = if num_children == 0 {
            self.bright_purple(" []".into()).to_string()
        } else {
            String::new()
        };

        // Append to string only if we are under the top level kind.
        self.result.push_str(
            format!(
                "{indent}{extra_head_indent}{}{extra_info}{suffix}\n",
                self.cyan(field_description.into())
            )
            .as_str(),
        );

        if num_children == 0 {
            return;
        }

        let extra_indent = if is_last { "    " } else { "│   " };
        let indent = String::from(indent) + extra_indent;
        let node_kind = self.get_node_kind(kind.to_string());
        match node_kind {
            NodeKind::Struct { members: expected_children }
            | NodeKind::Terminal { members: expected_children, .. } => {
                self.print_internal_struct(children, &expected_children, indent.as_str());
            }
            NodeKind::List { .. } => {
                for (i, child) in children.iter().enumerate() {
                    self.print_tree(
                        format!("child #{i}").as_str(),
                        child,
                        indent.as_str(),
                        i == num_children - 1,
                    );
                }
            }
            NodeKind::SeparatedList { .. } => {
                for (i, child) in children.iter().enumerate() {
                    let description = if i % 2 == 0 { "item" } else { "separator" };
                    self.print_tree(
                        format!("{description} #{}", i / 2).as_str(),
                        child,
                        indent.as_str(),
                        i == num_children - 1,
                    );
                }
            }
            _ => unreachable!("This should never happen"),
        }
    }

    /// Assumes children and expected children are non-empty of the same length.
    /// `under_top_level`: whether we are in a subtree of the top-level kind.
    fn print_internal_struct(
        &mut self,
        children: &[SyntaxNode],
        expected_children: &[Member],
        indent: &str,
    ) {
        let (last_child, non_last_children) = children.split_last().unwrap();
        let (last_expected_child, non_last_expected_children) =
            expected_children.split_last().unwrap();
        for (child, expected_child) in zip_eq(non_last_children, non_last_expected_children) {
            self.print_tree(&expected_child.name, child, indent, false);
        }
        self.print_tree(&last_expected_child.name, last_child, indent, true);
    }

    fn get_node_kind(&self, name: String) -> NodeKind {
        if let Some(node) = self.spec.iter().find(|x| x.name == name) {
            node.kind.clone()
        } else {
            panic!("Could not find spec for {name}")
        }
    }

    // Color helpers.
    fn bold(&self, text: ColoredString) -> ColoredString {
        if self.print_colors { text.bold() } else { text }
    }
    fn green(&self, text: ColoredString) -> ColoredString {
        if self.print_colors { text.green() } else { text }
    }
    fn red(&self, text: ColoredString) -> ColoredString {
        if self.print_colors { text.red() } else { text }
    }
    fn cyan(&self, text: ColoredString) -> ColoredString {
        if self.print_colors { text.cyan() } else { text }
    }
    fn blue(&self, text: ColoredString) -> ColoredString {
        if self.print_colors { text.blue() } else { text }
    }
    fn bright_purple(&self, text: ColoredString) -> ColoredString {
        if self.print_colors { text.bright_purple() } else { text }
    }
}

// TODO(yuval): autogenerate.
fn is_missing_kind(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::ExprMissing
            | SyntaxKind::WrappedArgListMissing
            | SyntaxKind::StatementMissing
            | SyntaxKind::ModuleItemMissing
            | SyntaxKind::TraitItemMissing
            | SyntaxKind::ImplItemMissing
    )
}
