use traits::{Minimum, Maximum, EntityId, AttributeId, TransactionId, V, Key, SIZE};


use arrow::datatypes::UnionMode;
use im::{OrdSet};
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::option::Option::Some;
use std::result::Result::Ok;
use std::collections::BTreeSet;
use std::collections::Bound;
use std::sync::Arc;
use immutable_chunkmap::set::Set;
use immutable_chunkmap::map::Map;
use uuid::Uuid;



struct LeafNode<K, V, const B: usize> {
    keys: [K; B],
    vals: [V; B],

}

const uniqueIdentity: Key = Key(":db.unique/identity");
const uniqueValue: Key = Key(":db.unique/value");
const cardinalityOne: Key = Key(":db.cardinality/one");
const cardinalityMany: Key = Key(":db.cardinality/many");

#[derive(Clone)]
struct Attribute {
    e: EntityId,
    ident: Key,
    cardinality: Key,
    value_type: Key, 
    component: bool,
    unique: Key,
}