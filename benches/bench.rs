use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_vcf_stats(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-vcf-stats");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let vcf = manifest.join("tests/golden/small.vcf");
    c.bench_function("rsomics-vcf-stats golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .arg(vcf.to_str().unwrap())
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_vcf_stats);
criterion_main!(benches);
