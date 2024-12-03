use aoclib::{input, output};

type Report = Vec<i32>;
type ReportVec = Vec<Report>;

fn main() {
    let input = input("input");

    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    p1(&reports);
    p2(&reports);
}

/// Returns number of unsafe reports.
#[inline]
fn p1(reports: &ReportVec) {
    let safe = reports.iter().filter(|r| is_safe(r)).count();
    output(safe);
}

#[inline]
fn p2(reports: &ReportVec) {
    let safe = reports.iter().filter(|r| is_semi_safe(r)).count();
    output(safe);
}

fn is_semi_safe(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }
    // brute force sucks, but I am struggling to find something better
    for i in 0..report.len() {
        let report_cut: Report = report[..i]
            .iter()
            .chain(report[i + 1..].iter())
            .copied()
            .collect();
        if is_safe(&report_cut) {
            return true;
        }
    }
    false
}

/// Returns true if the report is safe.
fn is_safe(report: &Report) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|x| x[1] - x[0]).collect();
    diffs.iter().all(|diff| (1..=3).contains(diff))
        || diffs.iter().all(|diff| (-3..=-1).contains(diff))
}
