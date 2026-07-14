use chess_kit_collections::{Map, Stack};
use criterion::Criterion;
use std::hint::black_box;
use std::mem::{align_of, size_of};

use crate::fixtures::SplitU64Hasher;

const MIB: usize = 1024 * 1024;

#[derive(Clone, Copy)]
pub struct LayoutReport {
    name: &'static str,
    requested_mib: usize,
    type_size: usize,
    type_align: usize,
    buckets: usize,
    capacity: usize,
    payload_bytes: usize,
    requested_bytes: usize,
}

impl LayoutReport {
    #[inline]
    fn overhead_bytes(self) -> isize {
        self.requested_bytes as isize - self.payload_bytes as isize
    }

    #[inline]
    fn payload_efficiency(self) -> f64 {
        if self.requested_bytes == 0 {
            0.0
        } else {
            self.payload_bytes as f64 / self.requested_bytes as f64
        }
    }
}

pub fn stack_report<T, const CAP: usize>(name: &'static str) -> LayoutReport
where
    T: chess_kit_collections::Copyable,
{
    LayoutReport {
        name,
        requested_mib: 0,
        type_size: size_of::<Stack<T, CAP>>(),
        type_align: align_of::<Stack<T, CAP>>(),
        buckets: 0,
        capacity: CAP,
        payload_bytes: CAP * size_of::<T>(),
        requested_bytes: 0,
    }
}

pub fn map_report<V>(name: &'static str, requested_mib: usize) -> LayoutReport
where
    V: chess_kit_collections::Value,
{
    let map = Map::<u64, V, SplitU64Hasher>::new(requested_mib);
    LayoutReport {
        name,
        requested_mib,
        type_size: size_of::<Map<u64, V, SplitU64Hasher>>(),
        type_align: align_of::<Map<u64, V, SplitU64Hasher>>(),
        buckets: map.buckets(),
        capacity: map.capacity(),
        payload_bytes: map.capacity() * size_of::<V>(),
        requested_bytes: requested_mib * MIB,
    }
}

pub fn bench_layout_reports(
    c: &mut Criterion,
    benchmark_name: &'static str,
    reports: &[LayoutReport],
) {
    print_report_table(benchmark_name, reports);

    c.bench_function(benchmark_name, |b| {
        b.iter(|| {
            let mut acc = 0_usize;
            for report in reports {
                acc ^= black_box(report.type_size);
                acc ^= black_box(report.capacity);
                acc ^= black_box(report.payload_bytes);
            }
            black_box(acc)
        });
    });
}

fn print_report_table(benchmark_name: &str, reports: &[LayoutReport]) {
    println!();
    println!("layout report: {benchmark_name}");
    println!(
        "{:<24} {:>7} {:>10} {:>5} {:>10} {:>10} {:>12} {:>12} {:>12} {:>8}",
        "name",
        "req MiB",
        "type",
        "align",
        "buckets",
        "capacity",
        "payload",
        "requested",
        "overhead",
        "eff",
    );
    println!("{}", "-".repeat(124));

    for report in reports {
        println!(
            "{:<24} {:>7} {:>10} {:>5} {:>10} {:>10} {:>12} {:>12} {:>12} {:>8}",
            report.name,
            report.requested_mib,
            format_bytes(report.type_size),
            report.type_align,
            format_count(report.buckets),
            format_count(report.capacity),
            format_bytes(report.payload_bytes),
            format_bytes(report.requested_bytes),
            format_signed_bytes(report.overhead_bytes()),
            format_percent(report.payload_efficiency()),
        );
    }

    println!();
}

fn format_count(value: usize) -> String {
    let digits = value.to_string();
    let mut formatted = String::with_capacity(digits.len() + digits.len() / 3);
    let leading = digits.len() % 3;

    for (idx, ch) in digits.chars().enumerate() {
        if idx > 0 && (idx - leading).is_multiple_of(3) {
            formatted.push('_');
        }
        formatted.push(ch);
    }

    formatted
}

fn format_bytes(bytes: usize) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }

    const UNITS: [&str; 4] = ["B", "KiB", "MiB", "GiB"];
    let mut value = bytes as f64;
    let mut unit = 0;

    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{bytes} {}", UNITS[unit])
    } else if value >= 100.0 {
        format!("{value:.0} {}", UNITS[unit])
    } else if value >= 10.0 {
        format!("{value:.1} {}", UNITS[unit])
    } else {
        format!("{value:.2} {}", UNITS[unit])
    }
}

fn format_signed_bytes(bytes: isize) -> String {
    match bytes.cmp(&0) {
        std::cmp::Ordering::Less => format!("-{}", format_bytes(bytes.unsigned_abs())),
        std::cmp::Ordering::Equal => "0 B".to_string(),
        std::cmp::Ordering::Greater => format!("+{}", format_bytes(bytes as usize)),
    }
}

fn format_percent(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}
