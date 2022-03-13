#![recursion_limit = "512"]
#![feature(generic_associated_types)]

#[macro_use] extern crate shrinkwraprs;

use edn_rs::edn::{Vector};
use edn_rs::Edn;
use edn_rs::edn;
use im::{OrdSet};
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::option::Option::Some;
use std::result::Result::Ok;
use std::collections::BTreeSet;
use std::collections::Bound;
use immutable_chunkmap::set::Set;
use immutable_chunkmap::map::Map;
use uuid::Uuid;

pub mod traits {

    pub type EntityId = i64;
    pub type AttributeId = i64;
    pub type TransactionId = i64;

    #[derive(Shrinkwrap, Clone, Debug)]
    pub struct Key(pub &'static str);

    pub trait Minimum {
        fn minimum() -> Self;
    }
    
    pub trait Maximum {
        fn maximum() -> Self;
    }

    // this works because entity id et al are a type alias, not a newtype. 
    // these'll need to be updated if that changes.
    impl Minimum for i64 {
        fn minimum() -> i64 {
            std::i64::MIN
        }
    }

    impl Maximum for i64 {
        fn maximum() -> i64 {
            std::i64::MAX
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum V<'v> {
        MinimumValue,
        String(&'v str),
        EntityId(EntityId),
        Uuid(uuid::Uuid),
        I64(i64),
        Key,
        MaximumValue
    }

    impl <'v> Minimum for V<'v> {
        fn minimum() -> V<'v> {
            V::MinimumValue
        }
    }

    impl <'v> Maximum for V<'v> {
        fn maximum() -> V<'v> {
            V::MaximumValue
        }
    }

    pub trait Datom<'v> {
        fn e(self) -> EntityId;
        fn a(self) -> AttributeId;
        fn v(self) -> V<'v>;
        fn t(self) -> TransactionId; 
        fn added(self) -> bool;
    }
}

use traits::{Minimum, Maximum, EntityId, AttributeId, TransactionId, V, Key};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Datom<'d> {
    pub e: EntityId,
    pub a: AttributeId,
    pub v: V<'d>,
    pub t: TransactionId,
}

impl <'d> std::fmt::Debug for Datom <'d> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        f.write_str("#Datom[")?;
        ::std::fmt::Debug::fmt(&self.e, f)?;
        f.write_str(" ")?;
        ::std::fmt::Debug::fmt(&self.a, f)?;
        f.write_str(" ")?;
        ::std::fmt::Debug::fmt(&self.v, f)?;
        f.write_str(" ")?;
        ::std::fmt::Debug::fmt(&self.t, f)?;
        f.write_str("]")?;
        Ok(())
    }
}

impl <'d> Datom<'d> {
    pub fn new(
        e: EntityId, 
        a: AttributeId, 
        v: V<'d>, 
        t: TransactionId) -> Datom {
            Datom {
                e, a, v, t
            }
        }
}

impl <'d> traits::Datom<'d> for Datom<'d> {
    fn e(self) -> EntityId {
        self.e
    }
    fn a(self) -> AttributeId {
         self.a
    }
    fn v(self) -> V<'d> {
         self.v
    }
    fn t(self) -> TransactionId {
         self.t
    }
    fn added(self) -> bool {
         self.e >= 0
    }
}
#[derive(Shrinkwrap, Clone, PartialEq, Eq)]
struct EAVTDatom<'d> { datom: Datom<'d> }

impl <'d> std::fmt::Debug for EAVTDatom <'d> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.datom, f)
    }
}

impl <'d> From<Datom<'d>> for EAVTDatom<'d> {
    fn from(datom: Datom<'d>) -> EAVTDatom<'d>{
        EAVTDatom{datom}
    }
}

impl PartialOrd for EAVTDatom<'_> {
    fn partial_cmp(&self, other: &EAVTDatom) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for EAVTDatom<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.datom.e.cmp(&other.datom.e);
        if ord == Equal {
            let ord = self.datom.a.cmp(&other.datom.a);
            if ord == Equal {
                let ord = self.datom.v.cmp(&other.datom.v);
                if ord == Equal {
                    self.datom.t.cmp(&other.datom.t)
                } else {
                    ord
                }
            } else {
                ord
            }
        } else {
            ord
        }
    }
}

#[derive(Shrinkwrap, Clone)]
struct EAVTIndex<'i> {
    index: Set::<EAVTDatom<'i>>,
}

impl <'index> EAVTIndex<'index> {
    fn new() -> EAVTIndex<'index>{
        EAVTIndex{index: Set::new()}
    }

    fn insert(self, datom: Datom<'index>) -> EAVTIndex {
        EAVTIndex{
            index: self.index.insert(EAVTDatom::from(datom)).0
        }
    }

    fn select_e(&self, e: EntityId) -> impl Iterator<Item=Datom> {
        let min = EAVTDatom::from(Datom::new(e, AttributeId::minimum(), V::minimum(), TransactionId::minimum()));
        let max = EAVTDatom::from(Datom::new(e, AttributeId::maximum(), V::maximum(), TransactionId::maximum()));
        let range = self.index.range(Bound::Included(min), Bound::Included(max)).map(|eavt_datom| eavt_datom.datom);
        range
    }
    fn select_ea(&self, e: EntityId, a: AttributeId) ->  impl Iterator<Item=Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, V::minimum(), TransactionId::minimum()));
        let max = EAVTDatom::from(Datom::new(e, a, V::maximum(), TransactionId::maximum()));
        let range = self.index.range(Bound::Included(min), Bound::Included(max)).map(|eavt_datom| eavt_datom.datom);
        range
    }
    fn select_eav(&self, e: EntityId, a: AttributeId, v: V<'index>) ->  impl Iterator<Item=Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, v, TransactionId::minimum()));
        let max = EAVTDatom::from(Datom::new(e, a, v, TransactionId::maximum()));
        let range = self.index.range(Bound::Included(min), Bound::Included(max)).map(|eavt_datom| eavt_datom.datom);
        range
    }
    fn select_eavt(&self, e: EntityId, a: AttributeId, v: V<'index>, t: TransactionId) -> impl Iterator<Item=Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, v, t));
        let max = EAVTDatom::from(Datom::new(e, a, v, t));
        let range = self.index.range(Bound::Included(min), Bound::Included(max)).map(|eavt_datom| eavt_datom.datom);
        range
    }
}

#[derive(Shrinkwrap, Clone)]
struct AEVTIndex<'i> {
    // index: BTreeSet::<AEVTDatom<'i>>,
    index: Set::<AEVTDatom<'i>>,
}

impl <'index> AEVTIndex<'index> {
    fn new() -> AEVTIndex<'index> {
        AEVTIndex{index: Set::new()}
    }

    fn insert(self, datom: Datom<'index>) -> AEVTIndex {
        AEVTIndex {
            index: self.index.insert(AEVTDatom::from(datom)).0,
        }
    }

    fn select_a(&self, a: AttributeId) -> impl Iterator<Item=Datom> {
        let min = AEVTDatom::from(Datom::new(EntityId::minimum(), a, V::minimum(), TransactionId::minimum()));
        let max = AEVTDatom::from(Datom::new(EntityId::maximum(), a, V::maximum(), TransactionId::maximum()));
        let range = self.index.range(Bound::Included(min), Bound::Included(max)).map(|aevt_datom| aevt_datom.datom);
        range
    }
    fn select_ae(&self, a: AttributeId, e: EntityId) -> impl Iterator<Item=Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, V::minimum(), TransactionId::minimum()));
        let max = AEVTDatom::from(Datom::new(e, a, V::maximum(), TransactionId::maximum()));
        let range = self.index.range(Bound::Included(min), Bound::Included(max)).map(|aevt_datom| aevt_datom.datom);
        range
    }
    fn select_aev(&self, a: AttributeId,  e: EntityId, v: V<'index>) ->  impl Iterator<Item=Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, v, TransactionId::minimum()));
        let max = AEVTDatom::from(Datom::new(e, a, v, TransactionId::maximum()));
        let range = self.index.range(Bound::Included(min), Bound::Included(max)).map(|aevt_datom| aevt_datom.datom);
        range
    }
    fn select_aevt(&self, a: AttributeId,  e: EntityId, v: V<'index>, t: TransactionId) -> impl Iterator<Item=Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, v, t));
        let max = AEVTDatom::from(Datom::new(e, a, v, t));
        let range = self.index.range(Bound::Included(min), Bound::Included(max)).map(|aevt_datom| aevt_datom.datom);
        range
    }
}

#[derive(Shrinkwrap, Clone, PartialEq, Eq)]
struct AEVTDatom<'d> { datom: Datom<'d> }

impl <'d> std::fmt::Debug for AEVTDatom <'d> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.datom, f)
    }
}

impl <'d> From<Datom<'d>> for AEVTDatom<'d> {
    fn from(datom: Datom<'d>) -> AEVTDatom<'d>{
        AEVTDatom{datom}
    }
}

impl PartialOrd for AEVTDatom<'_> {
    fn partial_cmp(&self, other: &AEVTDatom) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for AEVTDatom<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.datom.a.cmp(&other.datom.a);
        if ord == Equal {
            let ord = self.datom.e.cmp(&other.datom.e);
            if ord == Equal {
                let ord = self.datom.v.cmp(&other.datom.v);
                if ord == Equal {
                    self.datom.t.cmp(&other.datom.t)
                } else {
                    ord
                }
            } else {
                ord
            }
        } else {
            ord
        }
    }
}

#[derive(Clone)]
struct DatabaseSnapshot<'snapshot> {
    eavt: EAVTIndex<'snapshot>,
    aevt: AEVTIndex<'snapshot>,
    idents: Map<EntityId, Key>,
    attributes: Map<EntityId, Attribute>
}

impl <'snapshot> DatabaseSnapshot<'snapshot> {
    fn new() -> DatabaseSnapshot<'snapshot> {
        DatabaseSnapshot {
            eavt: EAVTIndex::new(),
            aevt: AEVTIndex::new(),
            idents: Map::new(),
            attributes: Map::new(),
        }
    }

    fn insert(self, datom: Datom<'snapshot>) -> DatabaseSnapshot {
        DatabaseSnapshot {
            eavt: self.eavt.insert(datom),
            aevt: self.aevt.insert(datom),
            idents: self.idents,
            attributes: self.attributes,
        }
    }

    fn select_e(&self, e: EntityId) -> impl Iterator<Item=Datom> { self.eavt.select_e(e) }
    fn select_ea(&self, e: EntityId, a: AttributeId) ->  impl Iterator<Item=Datom> { self.eavt.select_ea(e, a) }
    fn select_eav(&self, e: EntityId, a: AttributeId, v: V<'snapshot>) ->  impl Iterator<Item=Datom> { self.eavt.select_eav(e, a, v) }
    fn select_eavt(&self, e: EntityId, a: AttributeId, v: V<'snapshot>, t: TransactionId) -> impl Iterator<Item=Datom> { self.eavt.select_eavt(e, a, v, t) }

    fn select_a(&self, a: AttributeId) -> impl Iterator<Item=Datom> { self.aevt.select_a(a) }
    fn select_ae(&self, a: AttributeId, e: EntityId) -> impl Iterator<Item=Datom> { self.aevt.select_ae(a,e) }
    fn select_aev(&self, a: AttributeId,  e: EntityId, v: V<'snapshot>) ->  impl Iterator<Item=Datom> { self.aevt.select_aev(a,e,v) }
    fn select_aevt(&self, a: AttributeId,  e: EntityId, v: V<'snapshot>, t: TransactionId) -> impl Iterator<Item=Datom> { self.aevt.select_aevt(a,e,v,t) }
}

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

fn main() {
    let mut snapshot = DatabaseSnapshot::new();
    let mut snapshot1 : Option<DatabaseSnapshot> = None; 

    for eid in 0..5 {
        for aid in 0..5 {
            for v in 0..5 {
                let datom = Datom::new(eid, aid, V::I64(v), 1);
                snapshot = snapshot.insert(datom);
            }
        }
        if eid == 1 {
            snapshot1 = Some(snapshot.clone());
        }
    }

    println!("====== snapshot @ 5 ===");

    snapshot.select_a(3).for_each(
        |datom| println!("select a: {:?}", datom)
    );

    snapshot.select_ae(3 ,3).for_each(
        |datom| println!("select ae: {:?}", datom)
    );

    snapshot.select_aev(3, 3, V::I64(3)).for_each(
        |datom| println!("select aev: {:?}", datom)
    );

    snapshot.select_aevt(3, 3, V::I64(3), 1).for_each(
        |datom| println!("select aevt: {:?}", datom)
    );

    
    if let Some(snapshot1) = snapshot1 {
        println!("====== snapshot @ 1 ===");
        snapshot1.select_a(3).for_each(
            |datom| println!("select a: {:?}", datom)
        );
    
        snapshot1.select_ae(3 ,3).for_each(
            |datom| println!("select ae: {:?}", datom)
        );
    
        snapshot1.select_aev(3, 3, V::I64(3)).for_each(
            |datom| println!("select aev: {:?}", datom)
        );
    
        snapshot1.select_aevt(3, 3, V::I64(3), 1).for_each(
            |datom| println!("select aevt: {:?}", datom)
        );
    }
}
