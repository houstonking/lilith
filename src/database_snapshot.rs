use std::{time::Instant, collections::HashMap};

use crate::{
    datom::Datom,
    indexes::{AEVTIndex, EAVTIndex},
    AttributeId, EntityId, TransactionId, V, Key, pull::Pattern,
};

#[derive(Clone)]

pub struct DatabaseSnapshot {
    eavt: EAVTIndex,
    aevt: AEVTIndex,
    // idents: Map<EntityId, Key, SIZE>,
    // attributes: Map<EntityId, Attribute, SIZE>
}

// TODO: This is some overload-like behavior in Eva that has a 
// clunky translation. I'm not sure what the "best" way to represent
// the flexibility here is. In usage, it's a lot like "Into"/"From", 
// but the coercion to the desired type requires a snapshot.
pub enum Time {
    Instant(Instant),
    TransactionId(TransactionId),
}

// TODO: This is some overload-like behavior in Eva that has a 
// clunky translation. I'm not sure what the "best" way to represent
// the flexibility here is. In usage, it's a lot like "Into"/"From", 
// but the coercion to the desired type requires a snapshot.
pub enum Identity {
    EntityId(EntityId),
    LookupRef(LookupRef),
    Keyword(Key),
}

pub struct Attribute {}
pub struct LookupRef {}
pub struct Entity {}

pub enum Value {
    V(V), 
    Vec(Vec<Value>),
    Map(HashMap<Key, Value>),
}

pub struct HistorySnapshot {}

#[allow(unused_variables)]
impl DatabaseSnapshot {
    #[must_use]
    pub fn new() -> DatabaseSnapshot {
        DatabaseSnapshot {
            eavt: EAVTIndex::new(),
            aevt: AEVTIndex::new(),
            // idents: Map::new(),
            // attributes: Map::new(),
        }
    }

    #[must_use]
    pub fn insert(self, datom: Datom) -> Self {
        DatabaseSnapshot {
            eavt: self.eavt.insert(datom.clone()),
            aevt: self.aevt.insert(datom),
            // idents: self.idents,
            // attributes: self.attributes,
        }
    }

    // #region Datoms API 
    pub fn scan_eavt(&self) -> impl Iterator<Item = &Datom> {
        self.eavt.scan()
    }
    pub fn select_e(&self, e: EntityId) -> impl Iterator<Item = &Datom> {
        self.eavt.select_e(e)
    }
    pub fn select_ea(&self, e: EntityId, a: AttributeId) -> impl Iterator<Item = &Datom> {
        self.eavt.select_ea(e, a)
    }
    pub fn select_eav(
        &self,
        e: EntityId,
        a: AttributeId,
        v: &V,
    ) -> impl Iterator<Item = &Datom> {
        self.eavt.select_eav(e, a, v)
    }
    pub fn select_eavt(
        &self,
        e: EntityId,
        a: AttributeId,
        v: &V,
        t: TransactionId,
    ) -> impl Iterator<Item = &Datom> {
        self.eavt.select_eavt(e, a, v, t)
    }

    pub fn scan_aevt(&self) -> impl Iterator<Item = &Datom> {
        self.aevt.scan()
    }
    pub fn select_a(&self, a: AttributeId) -> impl Iterator<Item = &Datom> {
        self.aevt.select_a(a)
    }
    pub fn select_ae(&self, a: AttributeId, e: EntityId) -> impl Iterator<Item = &Datom> {
        self.aevt.select_ae(a, e)
    }
    pub fn select_aev(
        &self,
        a: AttributeId,
        e: EntityId,
        v: &V,
    ) -> impl Iterator<Item = &Datom> {
        self.aevt.select_aev(a, e, v)
    }
    pub fn select_aevt(
        &self,
        a: AttributeId,
        e: EntityId,
        v: &V,
        t: TransactionId,
    ) -> impl Iterator<Item = &Datom> {
        self.aevt.select_aevt(a, e, v, t)
    }
    // endregion

    pub fn as_of(&self, t: &Time) -> Self {
        unimplemented!()
    }
    pub fn as_of_t(&self) -> &Time {
        unimplemented!()
    }
    pub fn attribute(&self, attr_id: &Identity) -> &Attribute {
        unimplemented!()
    }
    pub fn basis_t(&self) -> &Time {
        unimplemented!()
    }
    pub fn ent_id(&self, ident: Identity) -> EntityId {
        unimplemented!()
    }
    pub fn entity(&self, eid: EntityId) -> Entity {
        unimplemented!()
    }
    pub fn filter<P>(&self, predicate: P) -> Self 
    where 
        P: Fn(&Datom) -> bool,
    {
        unimplemented!()
    }
    pub fn history(&self) -> HistorySnapshot {
        unimplemented!()
    }
    pub fn ident(&self, eid: EntityId) -> Identity {
        unimplemented!()
    }
    pub fn pull(&self, p: &Pattern, eid: EntityId) -> Value {
        unimplemented!()
    }

}

impl Default for DatabaseSnapshot {
    fn default() -> Self {
        Self::new()
    }
}
