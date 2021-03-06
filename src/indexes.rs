use std::ops::Bound;

use immutable_chunkmap::set::Set;

use crate::{datom::Datom, AttributeId, EntityId, Maximum, Minimum, TransactionId, SIZE, V};

use super::datom::{AEVTDatom, EAVTDatom};

#[derive(Shrinkwrap, Clone)]
pub(crate) struct EAVTIndex {
    index: Set<EAVTDatom, SIZE>,
}

impl EAVTIndex {
    pub(crate) fn new() -> EAVTIndex {
        EAVTIndex { index: Set::new() }
    }

    pub(crate) fn insert(self, datom: Datom) -> EAVTIndex {
        EAVTIndex {
            index: self.index.insert(EAVTDatom::from(datom)).0,
        }
    }

    pub(crate) fn scan(&self) -> impl Iterator<Item = &Datom> {
        self.index.into_iter().map(|datom| &datom.datom)
    }

    pub(crate) fn select_e(&self, e: EntityId) -> impl Iterator<Item = &Datom> {
        let min = EAVTDatom::from(Datom::new(
            e,
            AttributeId::minimum(),
            V::minimum(),
            TransactionId::minimum(),
        ));
        let max = EAVTDatom::from(Datom::new(
            e,
            AttributeId::maximum(),
            V::maximum(),
            TransactionId::maximum(),
        ));
        let range = self
            .index
            .range(Bound::Included(min), Bound::Included(max))
            .map(|datom| &datom.datom);
        range
    }
    pub(crate) fn select_ea(&self, e: EntityId, a: AttributeId) -> impl Iterator<Item = &Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, V::minimum(), TransactionId::minimum()));
        let max = EAVTDatom::from(Datom::new(e, a, V::maximum(), TransactionId::maximum()));
        let range = self
            .index
            .range(Bound::Included(min), Bound::Included(max))
            .map(|datom| &datom.datom);
        range
    }
    pub(crate) fn select_eav(
        &self,
        e: EntityId,
        a: AttributeId,
        v: &V,
    ) -> impl Iterator<Item = &Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, v.clone(), TransactionId::minimum()));
        let max = EAVTDatom::from(Datom::new(e, a, v.clone(), TransactionId::maximum()));
        let range = self
            .index
            .range(Bound::Included(min), Bound::Included(max))
            .map(|datom| &datom.datom);
        range
    }
    pub(crate) fn select_eavt(
        &self,
        e: EntityId,
        a: AttributeId,
        v: &V,
        t: TransactionId,
    ) -> impl Iterator<Item = &Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, v.clone(), t));
        let max = EAVTDatom::from(Datom::new(e, a, v.clone(), t));
        let range = self
            .index
            .range(Bound::Included(min), Bound::Included(max))
            .map(|datom| &datom.datom)
            ;
        range
    }
}

#[derive(Shrinkwrap, Clone)]

pub(crate) struct AEVTIndex {
    // index: BTreeSet::<AEVTDatom>,
    index: Set<AEVTDatom, SIZE>,
}

impl AEVTIndex {
    pub(crate) fn new() -> AEVTIndex {
        AEVTIndex { index: Set::new() }
    }

    pub(crate) fn insert(self, datom: Datom) -> AEVTIndex {
        AEVTIndex {
            index: self.index.insert(AEVTDatom::from(datom)).0,
        }
    }

    pub(crate) fn scan(&self) -> impl Iterator<Item = &Datom> {
        self.index.into_iter().map(|datom| &datom.datom)
    }

    pub(crate) fn select_a(&self, a: AttributeId) -> impl Iterator<Item = &Datom> {
        let min = AEVTDatom::from(Datom::new(
            EntityId::minimum(),
            a,
            V::minimum(),
            TransactionId::minimum(),
        ));
        let max = AEVTDatom::from(Datom::new(
            EntityId::maximum(),
            a,
            V::maximum(),
            TransactionId::maximum(),
        ));
        let range = self
            .index
            .range(Bound::Included(min), Bound::Included(max))
            .map(|datom| &datom.datom);
        range
    }
    pub(crate) fn select_ae(&self, a: AttributeId, e: EntityId) -> impl Iterator<Item = &Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, V::minimum(), TransactionId::minimum()));
        let max = AEVTDatom::from(Datom::new(e, a, V::maximum(), TransactionId::maximum()));
        let range = self
            .index
            .range(Bound::Included(min), Bound::Included(max))
            .map(|datom| &datom.datom);
        range
    }
    pub(crate) fn select_aev(
        &self,
        a: AttributeId,
        e: EntityId,
        v: &V,
    ) -> impl Iterator<Item = &Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, v.clone(), TransactionId::minimum()));
        let max = AEVTDatom::from(Datom::new(e, a, v.clone(), TransactionId::maximum()));
        let range = self
            .index
            .range(Bound::Included(min), Bound::Included(max))
            .map(|datom| &datom.datom);
        range
    }
    pub(crate) fn select_aevt(
        &self,
        a: AttributeId,
        e: EntityId,
        v: &V,
        t: TransactionId,
    ) -> impl Iterator<Item = &Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, v.clone(), t));
        let max = AEVTDatom::from(Datom::new(e, a, v.clone(), t));
        let range = self
            .index
            .range(Bound::Included(min), Bound::Included(max))
            .map(|datom| &datom.datom);
        range
    }
}
