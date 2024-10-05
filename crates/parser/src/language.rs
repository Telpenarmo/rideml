use rowan::Language;

use crate::generated::syntax_kinds::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CamlotLanguage {}
impl Language for CamlotLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from_raw(raw.0)
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into_raw())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<CamlotLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<CamlotLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<CamlotLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<CamlotLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<CamlotLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<CamlotLanguage>;
