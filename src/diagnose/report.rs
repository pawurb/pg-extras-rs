use crate::diagnose::recommendation;
use crate::diagnose::run::CheckResult;
use prettytable::{Cell, Row as TableRow, Table};

pub fn render_diagnose_report(items: Vec<CheckResult>) {
    let term_width = textwrap::termwidth();
    let recommendation_width = term_width / 3; // need to adjust this to make the recommendation text wrap nicely

    let mut table = Table::new();

    let mut header_cell = Cell::new("Diagnose Report").style_spec("bH3");
    header_cell.align(prettytable::format::Alignment::CENTER);
    table.set_titles(TableRow::new(vec![header_cell]));

    table.add_row(row!["Check", "Message", "Recommendation"]);

    for item in items {
        let style = if item.ok { "Fg" } else { "Fr" };

        let status_and_name = format!("[{}] - {}", if item.ok { "√" } else { "x" }, item.check);

        // get the recommendation for the check
        let recommendation = if item.ok {
            "None".to_string()
        } else {
            let (header, details) = recommendation::Recommendations.get(&item.check).unwrap();
            // build the recommendation text by concatenating the header and details with bullet points
            format!(
                "{}:\n{}",
                header,
                details
                    .iter()
                    .map(|detail| format!("• {}", detail))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        };

        table.add_row(TableRow::new(vec![
            Cell::new(status_and_name.as_str()).style_spec(style),
            Cell::new(item.message.as_str()).style_spec(style),
            Cell::new(textwrap::fill(&recommendation, recommendation_width).as_str())
                .style_spec(style),
        ]));
    }

    table.printstd();
}
