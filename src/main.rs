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
use uuid::Uuid;

pub mod traits {

    pub type EntityId = i64;
    pub type AttributeId = i64;
    pub type TransactionId = i64;

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

use traits::{Minimum, Maximum, EntityId, AttributeId, TransactionId, V};

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
        return Ok(());
    }
}

impl <'d> Datom<'d> {
    pub fn new(
        e: EntityId, 
        a: AttributeId, 
        v: V<'d>, 
        t: TransactionId) -> Datom {
            return Datom {
                e, a, v: v, t
            }
        }
}

impl <'d> traits::Datom<'d> for Datom<'d> {
    fn e(self) -> EntityId {
        return self.e;
    }
    fn a(self) -> AttributeId {
        return self.a;
    }
    fn v(self) -> V<'d> {
        return self.v;
    }
    fn t(self) -> TransactionId {
        return self.t;
    }
    fn added(self) -> bool {
        return self.e >= 0;
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
                    return self.datom.t.cmp(&other.datom.t);
                } else {
                    return ord
                }
            } else {
                return ord
            }
        } else {
            return ord
        }
    }
}

#[derive(Debug)]
struct EAVTIndex<'i> {
    index: BTreeSet::<EAVTDatom<'i>>,
}

impl <'index> EAVTIndex<'index> {
    fn new() -> EAVTIndex<'index>{
        EAVTIndex{index: BTreeSet::new()}
    }

    fn insert(&mut self, datom: Datom<'index>) {
        self.index.insert(EAVTDatom::from(datom));
    }

    fn select_e(&self, e: EntityId) -> impl Iterator<Item=Datom> {
        let min = EAVTDatom::from(Datom::new(e, AttributeId::minimum(), V::minimum(), TransactionId::minimum()));
        let max = EAVTDatom::from(Datom::new(e, AttributeId::maximum(), V::maximum(), TransactionId::maximum()));
        let range = self.index.range(min..=max).map(|eavt_datom| eavt_datom.datom);
        range
    }
    fn select_ea(&self, e: EntityId, a: AttributeId) ->  impl Iterator<Item=Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, V::minimum(), TransactionId::minimum()));
        let max = EAVTDatom::from(Datom::new(e, a, V::maximum(), TransactionId::maximum()));
        let range = self.index.range(min..=max).map(|eavt_datom| eavt_datom.datom);
        range
    }
    fn select_eav(&self, e: EntityId, a: AttributeId, v: V<'index>) ->  impl Iterator<Item=Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, v, TransactionId::minimum()));
        let max = EAVTDatom::from(Datom::new(e, a, v, TransactionId::maximum()));
        let range = self.index.range(min..=max).map(|eavt_datom| eavt_datom.datom);
        range
    }
    fn select_eavt(&self, e: EntityId, a: AttributeId, v: V<'index>, t: TransactionId) -> impl Iterator<Item=Datom> {
        let min = EAVTDatom::from(Datom::new(e, a, v, t));
        let max = EAVTDatom::from(Datom::new(e, a, v, t));
        let range = self.index.range(min..=max).map(|eavt_datom| eavt_datom.datom);
        range
    }
}

struct AEVTIndex<'i> {
    index: BTreeSet::<AEVTDatom<'i>>,
}

impl <'index> AEVTIndex<'index> {
    fn new() -> AEVTIndex<'index>{
        AEVTIndex{index: BTreeSet::new()}
    }

    fn insert(&mut self, datom: Datom<'index>) {
        self.index.insert(AEVTDatom::from(datom));
    }

    fn select_a(&self, a: AttributeId) -> impl Iterator<Item=Datom> {
        let min = AEVTDatom::from(Datom::new(EntityId::minimum(), a, V::minimum(), TransactionId::minimum()));
        let max = AEVTDatom::from(Datom::new(EntityId::maximum(), a, V::maximum(), TransactionId::maximum()));
        let range = self.index.range(min..=max).map(|aevt_datom| aevt_datom.datom);
        range
    }
    fn select_ae(&self, a: AttributeId, e: EntityId) -> impl Iterator<Item=Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, V::minimum(), TransactionId::minimum()));
        let max = AEVTDatom::from(Datom::new(e, a, V::maximum(), TransactionId::maximum()));
        let range = self.index.range(min..=max).map(|aevt_datom| aevt_datom.datom);
        range
    }
    fn select_aev(&self, a: AttributeId,  e: EntityId, v: V<'index>) ->  impl Iterator<Item=Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, v, TransactionId::minimum()));
        let max = AEVTDatom::from(Datom::new(e, a, v, TransactionId::maximum()));
        let range = self.index.range(min..=max).map(|aevt_datom| aevt_datom.datom);
        range
    }
    fn select_aevt(&self, a: AttributeId,  e: EntityId, v: V<'index>, t: TransactionId) -> impl Iterator<Item=Datom> {
        let min = AEVTDatom::from(Datom::new(e, a, v, t));
        let max = AEVTDatom::from(Datom::new(e, a, v, t));
        let range = self.index.range(min..=max).map(|aevt_datom| aevt_datom.datom);
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
                    return self.datom.t.cmp(&other.datom.t);
                } else {
                    return ord
                }
            } else {
                return ord
            }
        } else {
            return ord
        }
    }
}

struct DatabaseSnapshot<'snapshot> {
    eavt: EAVTIndex<'snapshot>,
    aevt: AEVTIndex<'snapshot>,
}

fn main() {
    let mut eavt_index = EAVTIndex::new();
    let mut aevt_index = AEVTIndex::new();
    let mut vs = OrdSet::<V>::new();

    // for eid in 0..5 {
    //     let datom = Datom::new(eid, 10 - eid, V::String("foo"), 1);
    //     eavt_index.insert(datom);
    //     aevt_index.insert(AEVTDatom::from(datom));
    //     vs.insert(datom.v);
    //     vs.insert(V::EntityId(eid));
    //     vs.insert(V::I64(eid));
    //     vs.insert(V::Uuid(Uuid::new_v4()));
    // }

    for eid in 0..5 {
        for aid in 0..5 {
            for v in 0..5 {
                let datom = Datom::new(eid, aid, V::I64(v), 1);
                eavt_index.insert(datom);
                aevt_index.insert(datom);
            }
        }
    }

    // let base = base_schema();
    // for cmd in base_schema().iter().unwrap() {
        
    //     let datom = Datom::new(
    //         cmd.get(0).unwrap().to_int().unwrap(), 
    //         a: AttributeId, 
    //         v: V<'d>, 
    //         t: TransactionId)
    //     // eavt_index.insert(EAVTDatom::from(Datom::new(cmd. , cmd[1], cmd[2], 0)));
    // }


    println!("EAVT: {:?}", eavt_index);

    eavt_index.select_e(1).for_each(
        |datom| println!("select e: {:?}", datom)
    );

    eavt_index.select_ea(1, 1).for_each(
        |datom| println!("select ea: {:?}", datom)
    );

    eavt_index.select_eav(1, 1, V::I64(1)).for_each(
        |datom| println!("select eav: {:?}", datom)
    );

    eavt_index.select_eavt(1, 1, V::I64(1), 1).for_each(
        |datom| println!("select eavt: {:?}", datom)
    );

    aevt_index.select_a(3).for_each(
        |datom| println!("select a: {:?}", datom)
    );

    aevt_index.select_ae(3 ,3).for_each(
        |datom| println!("select ae: {:?}", datom)
    );

    aevt_index.select_aev(3, 3, V::I64(3)).for_each(
        |datom| println!("select aev: {:?}", datom)
    );

    aevt_index.select_aevt(3, 3, V::I64(3), 1).for_each(
        |datom| println!("select aevt: {:?}", datom)
    );

    // println!("AEVT: {:?}", aevt_index);
    // println!("Vs: {:?}", vs);
    // println!("base schema: {:?}", base_schema())
}
