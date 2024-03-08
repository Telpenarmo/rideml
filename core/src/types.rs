use std::borrow::Borrow;

use la_arena::{Arena, Idx};

pub type TypeIdx = Idx<Type>;

pub enum Type {
    Var { name: String },
    Alias { name: String, defn: TypeIdx },
    Arrow(TypeIdx, TypeIdx),
    Unit,
    Integer,
}

pub struct TypesDatabase {
    pub types: Arena<Type>,
}

impl TypesDatabase {
    pub fn equivalent(&self, left_idx: TypeIdx, right_idx: TypeIdx) -> bool {
        if left_idx == right_idx {
            true;
        }

        let left: &Type = self.types[left_idx].borrow();
        let right: &Type = self.types[right_idx].borrow();

        match (left, right) {
            (Type::Integer, Type::Integer) | (Type::Unit, Type::Unit) => true,

            (Type::Alias { name, defn }, _) => self.equivalent(*defn, right_idx),
            (_, Type::Alias { name, defn }) => self.equivalent(left_idx, *defn),

            (Type::Var { name: l_name }, Type::Var { name: r_name }) => l_name == r_name,

            (Type::Arrow(l_from, l_to), Type::Arrow(r_from, r_to)) => {
                self.equivalent(*l_from, *r_from) && self.equivalent(*l_to, *r_to)
            }

            _ => false,
        }
    }
}

fn nth_ident(mut n: u32) -> String {
    let mut chars: Vec<u8> = Vec::new();
    while n >= 26 {
        let offset = n % 26;
        let offset8: u8 = offset.try_into().unwrap();
        chars.push(b'a' + offset8);
        n -= offset;
        n /= 26;
        n -= 1;
    }
    let offset8: u8 = n.try_into().unwrap();
    chars.push(b'a' + offset8);
    chars.reverse();
    String::from_utf8(chars).unwrap()
}

mod tests {
    use crate::types::nth_ident;

    #[test]
    fn nth_ident_tests() {
        assert_eq!(nth_ident(0), "a");
        assert_eq!(nth_ident(1), "b");
        assert_eq!(nth_ident(25), "z");
        assert_eq!(nth_ident(26), "aa");    
        assert_eq!(nth_ident(27), "ab");
        assert_eq!(nth_ident(2 * 26), "ba");
        assert_eq!(nth_ident(2 * 26 + 1), "bb");
    }

    #[test]
    fn nth_ident_returns_different_values() {
        
        let mut seen = std::collections::HashSet::new();

        for i in 0..1000 {
            assert!(seen.insert(nth_ident(i)));
        }
    }   
}
