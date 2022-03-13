use crate::{indexes::{EAVTIndex, AEVTIndex}, datom::Datom, EntityId, AttributeId, V, TransactionId};

#[derive(Clone)]

pub struct DatabaseSnapshot<'snapshot> {
    eavt: EAVTIndex<'snapshot>,
    aevt: AEVTIndex<'snapshot>,
    // idents: Map<EntityId, Key, SIZE>,
    // attributes: Map<EntityId, Attribute, SIZE>
}

impl <'snapshot> DatabaseSnapshot<'snapshot> {



    #[must_use]
    pub fn new() -> DatabaseSnapshot<'snapshot> {
        DatabaseSnapshot {
            eavt: EAVTIndex::new(),
            aevt: AEVTIndex::new(),
            // idents: Map::new(),
            // attributes: Map::new(),
        }
    }

    #[must_use]
    pub fn insert(self, datom: Datom<'snapshot>) -> Self {
        DatabaseSnapshot {
            eavt: self.eavt.insert(datom),
            aevt: self.aevt.insert(datom),
            // idents: self.idents,
            // attributes: self.attributes,
        }
    }

    pub fn scan_eavt(&self) -> impl Iterator<Item=Datom> { self.eavt.scan() }
    pub fn scan_aevt(&self) -> impl Iterator<Item=Datom> { self.aevt.scan() }

    pub fn select_e(&self, e: EntityId) -> impl Iterator<Item=Datom> { self.eavt.select_e(e) }
    pub fn select_ea(&self, e: EntityId, a: AttributeId) ->  impl Iterator<Item=Datom> { self.eavt.select_ea(e, a) }
    pub fn select_eav(&self, e: EntityId, a: AttributeId, v: V<'snapshot>) ->  impl Iterator<Item=Datom> { self.eavt.select_eav(e, a, v) }
    pub fn select_eavt(&self, e: EntityId, a: AttributeId, v: V<'snapshot>, t: TransactionId) -> impl Iterator<Item=Datom> { self.eavt.select_eavt(e, a, v, t) }

    pub fn select_a(&self, a: AttributeId) -> impl Iterator<Item=Datom> { self.aevt.select_a(a) }
    pub fn select_ae(&self, a: AttributeId, e: EntityId) -> impl Iterator<Item=Datom> { self.aevt.select_ae(a,e) }
    pub fn select_aev(&self, a: AttributeId,  e: EntityId, v: V<'snapshot>) ->  impl Iterator<Item=Datom> { self.aevt.select_aev(a,e,v) }
    pub fn select_aevt(&self, a: AttributeId,  e: EntityId, v: V<'snapshot>, t: TransactionId) -> impl Iterator<Item=Datom> { self.aevt.select_aevt(a,e,v,t) }
}

impl<'snapshot> Default for DatabaseSnapshot<'snapshot> {
    fn default() -> Self {
    Self::new()
    }
    }