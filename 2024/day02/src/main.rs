use utils::parse_input;

const MIN_LEVEL_DIFFERENCE: i32 = 1;
const MAX_LEVEL_DIFFERENCE: i32 = 3;

fn parse_input_entries(input: String) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for entry in input.split('\n') {
        let report = entry
            .split(' ')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        reports.push(report);
    }

    reports
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let is_ascending = report[1] > report[0];

    for i in 0..report.len() - 1 {
        if (is_ascending && report[i + 1] <= report[i]) || (!is_ascending && report[i + 1] >= report[i]) {
            return false;
        }

        let diff = (report[i + 1] - report[i]).abs();
        if diff < MIN_LEVEL_DIFFERENCE || diff > MAX_LEVEL_DIFFERENCE {
            return false;
        }
    }

    true
}

fn problem1(reports: &Vec<Vec<i32>>) {
    let safe_reports_len = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();

    println!("Problem 1 -> {}", safe_reports_len);
}

fn problem2(reports: &Vec<Vec<i32>>) {
    let safe_reports_len = reports
        .iter()
        .filter(|report| {
            for i in 0..report.len() {
                let tmp_report = {
                    let mut tmp = report.to_vec();
                    tmp.remove(i);
                    tmp
                };

                if is_report_safe(tmp_report.as_ref()) {
                    return true;
                }
            }

            is_report_safe(report)
        })
        .count();

    println!("Problem 2 -> {}", safe_reports_len);
}

fn main() {
    let input = parse_input::<Vec<Vec<i32>>>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
