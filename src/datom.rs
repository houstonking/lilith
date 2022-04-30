use std::cmp::Ordering;

use crate::{AttributeId, Datom as DatomTrait, EntityId, TransactionId, V};

#[derive(Clone, PartialEq, Eq)]
pub struct Datom {
    pub e: EntityId,
    pub a: AttributeId,
    pub v: V,
    pub t: TransactionId,
}

impl std::fmt::Debug for Datom {
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

impl Datom {
    pub fn new(e: EntityId, a: AttributeId, v: V, t: TransactionId) -> Datom {
        Datom { e, a, v, t }
    }
}

impl DatomTrait for Datom {
    fn e(self) -> EntityId {
        self.e
    }
    fn a(self) -> AttributeId {
        self.a
    }
    fn v(self) -> V {
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
pub(crate) struct EAVTDatom {
    pub datom: Datom,
}

impl std::fmt::Debug for EAVTDatom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.datom, f)
    }
}

impl From<Datom> for EAVTDatom {
    fn from(datom: Datom) -> EAVTDatom {
        EAVTDatom { datom }
    }
}

impl PartialOrd for EAVTDatom {
    fn partial_cmp(&self, other: &EAVTDatom) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EAVTDatom {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.datom.e.cmp(&other.datom.e);
        if ord == Ordering::Equal {
            let ord = self.datom.a.cmp(&other.datom.a);
            if ord == Ordering::Equal {
                let ord = self.datom.v.cmp(&other.datom.v);
                if ord == Ordering::Equal {
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

#[derive(Shrinkwrap, Clone, PartialEq, Eq)]
pub(crate) struct AEVTDatom {
    pub datom: Datom,
}

impl std::fmt::Debug for AEVTDatom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.datom, f)
    }
}

impl From<Datom> for AEVTDatom {
    fn from(datom: Datom) -> AEVTDatom {
        AEVTDatom { datom }
    }
}

impl PartialOrd for AEVTDatom {
    fn partial_cmp(&self, other: &AEVTDatom) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AEVTDatom {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.datom.a.cmp(&other.datom.a);
        if ord == Ordering::Equal {
            let ord = self.datom.e.cmp(&other.datom.e);
            if ord == Ordering::Equal {
                let ord = self.datom.v.cmp(&other.datom.v);
                if ord == Ordering::Equal {
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
