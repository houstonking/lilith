
use lilith::{database_snapshot::DatabaseSnapshot, datom::Datom, V};

fn main() {
    // use arrow::datatypes::{UnionMode, DataType, Field, Schema};
    // use arrow::record_batch::RecordBatch;

    // let field_e = Field::new("e", DataType::Int64, false);
    // let field_a = Field::new("a", DataType::Int64, false);
    // let field_v = Field::new("v", DataType::Union(vec![], UnionMode::Dense), false);
    // let field_t = Field::new("t", DataType::Int64, false);

    // let eavt_schema = Schema::new(vec![field_e, field_a, field_v, field_t]);

    // let batch = RecordBatch::try_from_iter(Arc::new(schema),
    //     vec![] 
    // )

    let mut snapshot = DatabaseSnapshot::new();
    let mut snapshot1: Option<DatabaseSnapshot> = None;

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

    snapshot
        .select_a(3)
        .for_each(|datom| println!("select a: {:?}", datom));

    snapshot
        .select_ae(3, 3)
        .for_each(|datom| println!("select ae: {:?}", datom));

    snapshot
        .select_aev(3, 3, V::I64(3))
        .for_each(|datom| println!("select aev: {:?}", datom));

    snapshot
        .select_aevt(3, 3, V::I64(3), 1)
        .for_each(|datom| println!("select aevt: {:?}", datom));

    if let Some(snapshot1) = snapshot1 {
        println!("====== snapshot @ 1 ===");

        snapshot1
            .scan_aevt()
            .for_each(|datom| println!("scan aevt: {:?}", datom));

        snapshot1
            .select_a(3)
            .for_each(|datom| println!("select a: {:?}", datom));

        snapshot1
            .select_ae(3, 3)
            .for_each(|datom| println!("select ae: {:?}", datom));

        snapshot1
            .select_aev(3, 3, V::I64(3))
            .for_each(|datom| println!("select aev: {:?}", datom));

        snapshot1
            .select_aevt(3, 3, V::I64(3), 1)
            .for_each(|datom| println!("select aevt: {:?}", datom));
    }
}
