use rowan::{GreenNodeBuilder, Language};
use std::mem;

use crate::source::Token;
use crate::{event::Event, CamlotLanguage};
use crate::{Parse, SyntaxError, SyntaxKind};

pub(crate) struct Sink<'t, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'t [Token<'input>],
    cursor: usize,
    events: Vec<Event>,
    errors: Vec<SyntaxError>,
}

impl<'t, 'input> Sink<'t, 'input> {
    pub(crate) fn new(tokens: &'t [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
            errors: Vec::new(),
        }
    }

    pub(crate) fn finish(mut self) -> Parse {
        let mut idx = 0;
        while idx < self.events.len() {
            match mem::replace(&mut self.events[idx], Event::UnmatchedOpen) {
                Event::Open {
                    kind,
                    forward_parent,
                } => self.open_node(idx, kind, forward_parent),
                Event::Advance => self.token(),
                Event::Close | Event::CloseError => self.builder.finish_node(),
                Event::OpenError {
                    error,
                    forward_parent,
                } => {
                    self.open_node(idx, SyntaxKind::ERROR, forward_parent);
                    self.errors.push(SyntaxError { message: error });
                }
                Event::UnmatchedOpen => {}
            }
            idx += 1;

            if self.is_good_place_for_trivia(idx) {
                self.eat_trivia();
            }
        }

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn open_node(&mut self, idx: usize, kind: SyntaxKind, forward_parent: Option<usize>) {
        let mut kinds = vec![kind];

        let mut idx = idx;
        let mut forward_parent = forward_parent;

        // Walk through the forward parent of the forward parent and the forward parent
        // of that, and of that, etc. until we reach a StartNode event without a forward
        // parent.
        while let Some(fp) = forward_parent {
            idx += fp;

            forward_parent = if let Event::Open {
                kind,
                forward_parent,
            } = mem::replace(&mut self.events[idx], Event::UnmatchedOpen)
            {
                kinds.push(kind);
                forward_parent
            } else {
                unreachable!()
            };
        }

        for kind in kinds.into_iter().rev() {
            self.builder.start_node(CamlotLanguage::kind_to_raw(kind));
        }
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind.is_trivial() {
                break;
            }
            if token.kind == SyntaxKind::LEXING_ERROR {
                self.errors.push(SyntaxError {
                    message: format!("Lexing error: \"{}\"", token.text),
                });
            }
            self.token();
        }
    }

    fn token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.cursor];

        self.builder.token(CamlotLanguage::kind_to_raw(kind), text);

        self.cursor += 1;
    }

    // This function implements a heuristic for errors location.
    // In normal cases trivia are appended to the last opened node.
    // However, errors often are related to the preceding nodes.
    // For this reason we try to delay eating trivia until after the error.
    fn is_good_place_for_trivia(&self, idx: usize) -> bool {
        for event in &self.events[idx..] {
            match event {
                Event::Open { .. } | Event::UnmatchedOpen | Event::Advance => return true,
                Event::OpenError { .. } | Event::CloseError => return false,
                Event::Close => continue,
            }
        }

        true
    }
}
