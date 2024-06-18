use crate::{
    event::{ErrorPlacement, Event, ParseError},
    source::Source,
    SyntaxKind,
};

#[allow(dead_code)]
pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

/// Transform the source into events
impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    pub(crate) fn finish(self) -> Vec<Event> {
        self.events
    }

    /// Open a new node
    pub(crate) fn open(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::UnmatchedOpen);
        Marker::new(pos)
    }

    /// Close a node
    pub(crate) fn close(&mut self, marker: Marker, kind: SyntaxKind) -> CompletedMarker {
        let event_at_pos = &mut self.events[marker.pos];
        assert_eq!(*event_at_pos, Event::UnmatchedOpen);

        *event_at_pos = Event::Open {
            kind,
            forward_parent: None,
        };

        self.events.push(Event::Close);

        marker.complete()
    }

    /// Open a new node above the given marker's node
    pub(crate) fn open_before(&mut self, marker: CompletedMarker) -> Marker {
        let new_m = self.open();

        if let Event::Open {
            ref mut forward_parent,
            ..
        } = self.events[marker.pos]
        {
            *forward_parent = Some(new_m.pos - marker.pos);
        } else {
            unreachable!("Expected open event at {}", marker.pos);
        }

        new_m
    }

    /// Check if the current token is the given kind
    pub(crate) fn at(&mut self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    /// Check if the current token is any of the given kinds
    pub(crate) fn at_any(&mut self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    /// Get the current token, ignoring trivia
    fn current(&mut self) -> SyntaxKind {
        self.source.current()
    }

    /// Advance to the next token
    pub(crate) fn advance(&mut self) {
        self.source.bump();
        self.events.push(Event::Advance);
    }

    /// Advance if the current token is the given kind
    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Advance if the current token is the given kind, otherwise emit an error
    pub(crate) fn expect(&mut self, kind: SyntaxKind) {
        if self.eat(kind) {
            return;
        }

        let msg = format!("Expected {:#?} but found {:?}", kind, self.current());
        eprintln!("{msg}");
        self.error(msg, ErrorPlacement::PrevTokenEnd);
    }

    pub(crate) fn error(&mut self, message: String, location: ErrorPlacement) {
        self.events
            .push(Event::Error(ParseError { message, location }));
    }

    pub(crate) fn unexpected(&mut self) {
        let msg = format!("Unexpected token: {:#?}", self.current());
        self.error(msg, ErrorPlacement::NextToken);
        self.advance();
    }

    pub(crate) fn eat_error_until(&mut self, delimiters: TokenSet) -> CompletedMarker {
        let marker = self.open();
        let delimiters = delimiters.union(TokenSet::new(&[SyntaxKind::EOF]));
        loop {
            if delimiters.contains(self.current()) {
                break;
            }
            self.unexpected();
        }
        self.close(marker, SyntaxKind::ERROR)
    }
}

#[must_use]
pub(crate) struct Marker {
    pos: usize,
    completed: bool,
}

impl Marker {
    fn new(pos: usize) -> Self {
        Self {
            pos,
            completed: false,
        }
    }

    fn complete(mut self) -> CompletedMarker {
        self.completed = true;

        CompletedMarker { pos: self.pos }
    }
}

impl Drop for Marker {
    fn drop(&mut self) {
        assert!(
            !(!self.completed && !::std::thread::panicking()),
            "Marker dropped without completion"
        );
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CompletedMarker {
    pub(crate) pos: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct TokenSet([u64; 1]);

impl TokenSet {
    pub(crate) fn new(tokens: &[SyntaxKind]) -> Self {
        let mut set = Self([0; 1]);
        for &token in tokens {
            set.0[0] |= 1 << token as u64;
        }
        set
    }

    pub(crate) fn union(self, other: Self) -> Self {
        TokenSet([self.0[0] | other.0[0]])
    }

    pub(crate) fn contains(self, kind: SyntaxKind) -> bool {
        (self.0[0] & (1 << kind as u64)) != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check(
            crate::PrefixEntryPoint::Module,
            "",
            &expect![[r"
            MODULE@0..0
        "]],
        );
    }

    #[test]
    fn parse_whitespace() {
        check(
            crate::PrefixEntryPoint::Module,
            "   ",
            &expect![[r#"
                MODULE@0..3
                  WHITESPACE@0..3 "   "
            "#]],
        );
    }

    #[test]
    fn token_set() {
        let set = &super::TokenSet::new(&[super::SyntaxKind::EOF]);
        assert!(set.contains(super::SyntaxKind::EOF));
        assert!(!set.contains(super::SyntaxKind::IDENT));

        let set = &super::TokenSet::new(&[super::SyntaxKind::EOF, super::SyntaxKind::IDENT]);
        assert!(set.contains(super::SyntaxKind::EOF));
        assert!(set.contains(super::SyntaxKind::IDENT));
        assert!(!set.contains(super::SyntaxKind::COLON));
    }

    #[test]
    fn token_set_union() {
        let set = super::TokenSet::new(&[super::SyntaxKind::EOF])
            .union(super::TokenSet::new(&[super::SyntaxKind::IDENT]));
        assert!(set.contains(super::SyntaxKind::EOF));
        assert!(set.contains(super::SyntaxKind::IDENT));
        assert!(!set.contains(super::SyntaxKind::ARROW));
    }
}
