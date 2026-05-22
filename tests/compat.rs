use std::path::PathBuf;
use std::process::{Command, Stdio};

fn ours() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_rsomics-vcf-stats"))
}
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name)
}

fn bcftools_available() -> bool {
    Command::new("bcftools")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// ours emits `Label:\t<n>`; pull the count for a given label.
fn ours_count(out: &str, label: &str) -> i64 {
    out.lines()
        .find(|l| l.starts_with(label))
        .and_then(|l| l.split('\t').nth(1))
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or_else(|| panic!("ours missing {label}"))
}

/// bcftools stats emits `SN\t0\t<label>\t<n>`; pull the count for a label.
fn bcftools_sn(out: &str, label: &str) -> i64 {
    out.lines()
        .filter(|l| l.starts_with("SN\t"))
        .find(|l| l.contains(label))
        .and_then(|l| l.rsplit('\t').next())
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or_else(|| panic!("bcftools missing {label}"))
}

// Field-level compat: variant/SNP/indel counts must match `bcftools stats`
// (output formats differ, so compare parsed counts, not bytes).
#[test]
fn counts_match_bcftools() {
    if !bcftools_available() {
        eprintln!("skipping: bcftools not found");
        return;
    }
    let vcf = fixture("small.vcf");
    let ours_out =
        String::from_utf8(Command::new(ours()).arg(&vcf).output().unwrap().stdout).unwrap();
    let bcf_out = String::from_utf8(
        Command::new("bcftools")
            .arg("stats")
            .arg(&vcf)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    assert_eq!(
        ours_count(&ours_out, "Total variants:"),
        bcftools_sn(&bcf_out, "number of records:"),
        "variant count"
    );
    assert_eq!(
        ours_count(&ours_out, "SNPs:"),
        bcftools_sn(&bcf_out, "number of SNPs:"),
        "SNP count"
    );
    assert_eq!(
        ours_count(&ours_out, "Insertions:") + ours_count(&ours_out, "Deletions:"),
        bcftools_sn(&bcf_out, "number of indels:"),
        "indel count"
    );
}

#[test]
fn runs_with_fixture() {
    let out = Command::new(ours())
        .arg(fixture("small.vcf"))
        .output()
        .expect("spawn");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
