use std::cmp::Ordering;

use crate::{AttributeId, Datom as DatomTrait, EntityId, TransactionId, V};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Datom<'d> {
    pub e: EntityId,
    pub a: AttributeId,
    pub v: V<'d>,
    pub t: TransactionId,
}

impl<'d> std::fmt::Debug for Datom<'d> {
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

impl<'d> Datom<'d> {
    pub fn new(e: EntityId, a: AttributeId, v: V<'d>, t: TransactionId) -> Datom {
        Datom { e, a, v, t }
    }
}

impl<'d> DatomTrait<'d> for Datom<'d> {
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
pub(crate) struct EAVTDatom<'d> {
    pub datom: Datom<'d>,
}

impl<'d> std::fmt::Debug for EAVTDatom<'d> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.datom, f)
    }
}

impl<'d> From<Datom<'d>> for EAVTDatom<'d> {
    fn from(datom: Datom<'d>) -> EAVTDatom<'d> {
        EAVTDatom { datom }
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
pub(crate) struct AEVTDatom<'d> {
    pub datom: Datom<'d>,
}

impl<'d> std::fmt::Debug for AEVTDatom<'d> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.datom, f)
    }
}

impl<'d> From<Datom<'d>> for AEVTDatom<'d> {
    fn from(datom: Datom<'d>) -> AEVTDatom<'d> {
        AEVTDatom { datom }
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
