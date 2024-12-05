use std::io;

fn is_valid_report(report: &Vec<i64>) -> bool {
    if report[0] < report[1] { // increasing report
        for i in 0..(report.len() - 1) {
            let del = report[i + 1] - report[i];
            if del < 1 || del > 3 {
                return false;
            }
        }
    } else { // decreasing report
        for i in 0..(report.len() - 1) {
            let del = report[i] - report[i + 1];
            if del < 1 || del > 3 {
                return false;
            }
        }
    }

    true
}

#[allow(dead_code)]
fn part1(reports: Vec<Vec<i64>>) {
    let valid_reports = reports.iter().filter(|r| is_valid_report(r)).count();
    
    println!("{}", valid_reports);
}

fn part2(reports: Vec<Vec<i64>>) {
    let mut valid_reports = 0;
    
    for report in reports {
        if is_valid_report(&report) {
            valid_reports += 1;
        } else {
            for i in 0..report.len() {
                let new_report = report.iter().enumerate().filter(|(idx, _)| *idx != i).map(|(_, value)| *value).collect::<Vec<_>>();
                if is_valid_report(&new_report) {
                    valid_reports += 1;
                    break;
                }
            }
        }
    }

    println!("{}", valid_reports);
}


fn main() {
    let mut line = String::new();
    let mut reports = Vec::new();

    while io::stdin().read_line(&mut line).is_ok_and(|v| v != 0) {
        let line = std::mem::take(&mut line);
        let parts = line.split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>();
        reports.push(parts);
    }

    // part1(reports);
    part2(reports);
}
