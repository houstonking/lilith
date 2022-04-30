
use im::HashMap;

use crate::{Key, V};


// pattern             = [attr-spec+]
pub struct Pattern {
    specs: Vec<AttrSpec>,
}

impl Pattern {
    pub fn new(specs: Vec<AttrSpec>) -> Self {
        Pattern{
            specs
        }
    }
}

// attr-name           = an edn keyword that names an attr
type AttrName = Key;

// recursion-limit     = positive-number | '...'
pub enum RecursionLimit {
    Bounded(u32),
    Unbounded,
}

// attr-spec           = attr-name | wildcard | map-spec | attr-expr
pub enum AttrSpec {
    Wildcard,
    Attribute(Attribute),
    Recursion(Recursion),
}

// pub enum PatternOrRecursionLimit {
//     Pattern(Pattern),
//     RecursionLimit(RecursionLimit),
// }

// map-spec            = { ((attr-name | attr-expr) (pattern | recursion-limit))+ }
// pub struct MapSpec {
//     data: HashMap<Attribute, PatternOrRecursionLimit>
// }

// attr-option         = as-expr | limit-expr | default-expr
pub struct Attribute {
    name: Key,
    rename: Option<V>,
    limit: Option<u32>,
    default: Option<V>,
    reverse: bool,
}

pub struct Recursion {
    // The attribute to traverse and recur upon. Must be a reference-type attribute.
    target: Attribute,
    // The maximum recursion depth to allow, if None, recursion is *unbounded*.
    limit: RecursionLimit,
    // The pull pattern to apply to the entities found as part of the recursion. 
    // If None, the pattern in which this recursion is nested will be used.
    pattern: Option<Pattern>,
}

impl Recursion {
    pub fn new(target: Attribute, limit: RecursionLimit) -> Self {
        Recursion{target, limit, pattern: None}
    }
}

impl Attribute {
    pub fn new(name: Key) -> Attribute {
        Attribute { name, rename: None, limit: Some(1000), default: None, reverse: false}
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    } 
}

mod test {
    use crate::pull::*;

    fn test() {
        let db_id = Key(":db/id".to_string());
        let artist_name = Key(":artist/name".to_string());
        let artist_gid = Key(":artist/gid".to_string());
        let artist_country = Key(":artist/country".to_string());

        let track_name = Key(":track/name".to_string());

        let attribute_names = Pattern::new(
            vec![
                AttrSpec::Attribute(Attribute::new(artist_name)),
                AttrSpec::Attribute(Attribute::new(artist_gid))
            ]
        );

        let reverse_attribute = Pattern::new(
            vec![
                AttrSpec::Attribute(Attribute::new(artist_country).reverse())
            ]
        );

        let map_spec = Pattern::new(
            vec![
                AttrSpec::Attribute(Attribute::new(track_name)),
                AttrSpec::Recursion(
                    Recursion::new()
                )
            ]
        )
    }

}