use fme_fer::BaselinePath;
use fme_fer::baseline_v1::{PROFILE_ID, evaluate};
use std::env::consts::{ARCH, OS};
use std::hint::black_box;
use std::process::Command;
use std::str::FromStr;
use std::time::Instant;

const DEPTHS: [usize; 10] = [8, 16, 32, 64, 128, 256, 384, 512, 768, 1000];

#[derive(Clone, Copy)]
enum WorkloadKind {
    RepeatedF0,
    RepeatedF1,
    Alternating01,
    Alternating10,
    Mixed,
}

impl WorkloadKind {
    const ALL: [Self; 5] = [
        Self::RepeatedF0,
        Self::RepeatedF1,
        Self::Alternating01,
        Self::Alternating10,
        Self::Mixed,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::RepeatedF0 => "repeated_f0",
            Self::RepeatedF1 => "repeated_f1",
            Self::Alternating01 => "alternating_01",
            Self::Alternating10 => "alternating_10",
            Self::Mixed => "mixed_00101101",
        }
    }

    fn path_string(self, depth: usize) -> String {
        match self {
            Self::RepeatedF0 => "0".repeat(depth),
            Self::RepeatedF1 => "1".repeat(depth),
            Self::Alternating01 => "01".chars().cycle().take(depth).collect(),
            Self::Alternating10 => "10".chars().cycle().take(depth).collect(),
            Self::Mixed => "00101101".chars().cycle().take(depth).collect(),
        }
    }
}

struct Workload {
    name: &'static str,
    depth: usize,
    path: BaselinePath,
    warmup_iterations: u64,
    measured_iterations: u64,
}

fn iteration_policy(depth: usize) -> (u64, u64) {
    match depth {
        8 => (20_000, 1_000_000),
        16 => (10_000, 500_000),
        32 => (5_000, 250_000),
        64 => (2_000, 100_000),
        128 => (1_000, 50_000),
        256 => (500, 25_000),
        384 => (300, 15_000),
        512 => (200, 10_000),
        768 => (100, 5_000),
        1000 => (100, 5_000),
        _ => unreachable!("benchmark depth is not part of the fixed Phase 005 matrix"),
    }
}

fn build_workloads() -> Vec<Workload> {
    let mut workloads = Vec::new();

    for kind in WorkloadKind::ALL {
        for depth in DEPTHS {
            let source = kind.path_string(depth);
            let path = BaselinePath::from_str(&source).expect("fixed benchmark path must be valid");

            assert_eq!(
                path.len(),
                depth,
                "fixed benchmark path depth must match declared depth"
            );

            let (warmup_iterations, measured_iterations) = iteration_policy(depth);

            workloads.push(Workload {
                name: kind.name(),
                depth,
                path,
                warmup_iterations,
                measured_iterations,
            });
        }
    }

    workloads
}

fn rustc_version() -> String {
    Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_else(|| "unavailable".to_owned())
}

fn run_workload(workload: &Workload) {
    for _ in 0..workload.warmup_iterations {
        let state = evaluate(black_box(&workload.path))
            .expect("fixed benchmark workload must evaluate successfully");
        black_box(state);
    }

    let started = Instant::now();

    for _ in 0..workload.measured_iterations {
        let state = evaluate(black_box(&workload.path))
            .expect("fixed benchmark workload must evaluate successfully");
        black_box(state);
    }

    let elapsed = started.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();

    let evaluations_per_second = workload.measured_iterations as f64 / elapsed_seconds;

    let transforms_per_second = evaluations_per_second * workload.depth as f64;

    println!(
        concat!(
            "profile={profile} ",
            "workload={workload} ",
            "depth={depth} ",
            "warmup_iterations={warmup} ",
            "measured_iterations={measured} ",
            "elapsed_seconds={elapsed:.6} ",
            "evaluations_per_second={evals:.3} ",
            "transforms_per_second={transforms:.3}"
        ),
        profile = PROFILE_ID,
        workload = workload.name,
        depth = workload.depth,
        warmup = workload.warmup_iterations,
        measured = workload.measured_iterations,
        elapsed = elapsed_seconds,
        evals = evaluations_per_second,
        transforms = transforms_per_second,
    );
}

fn main() {
    println!("FME PHASE 005 BASELINE FER BENCHMARK");
    println!("profile={PROFILE_ID}");
    println!("cargo_profile=bench");
    println!("target_arch={ARCH}");
    println!("target_os={OS}");
    println!("rust_toolchain={}", rustc_version());
    println!("timing_scope=exact_complete_path_fer_evaluation_only");
    println!("path_parsing_inside_timed_region=false");
    println!("fixture_loading_inside_timed_region=false");
    println!(
        "measurement_warning=FER transforms are topology transforms, not events or transactions"
    );
    println!();

    let workloads = build_workloads();

    for workload in &workloads {
        run_workload(workload);
    }
}
