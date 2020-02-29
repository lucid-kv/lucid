use criterion::{criterion_group, criterion_main, Criterion};

use lucid::kvstore::KvStore;

fn set_10_kb_data_without_encryption(c: &mut Criterion) {
    let kv = KvStore::new(None);

    let data = [42u8; 10000];

    c.bench_function("Set 10KB (no encrytion)", |b| {
        b.iter(|| kv.set("bench_one".to_string(), data.to_vec()))
    });
}
fn set_10_kb_data(c: &mut Criterion) {
    let kv = KvStore::new(Some([
        "123456789012345678901234123456789012345678901234",
        "f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
    ]));

    let data = [42u8; 10000];

    c.bench_function("Set 10KB", |b| {
        b.iter(|| kv.set("bench_one".to_string(), data.to_vec()))
    });
}

criterion_group!(benches, set_10_kb_data, set_10_kb_data_without_encryption);
criterion_main!(benches);
