pub fn display_personalities(personalities: &Vec<(String, f32)>) {
    if personalities.is_empty() {
        println!("No personalities found.");
        return;
    }

    const TITLE_HEADER: &str = "Title";
    const PERCENTAGE_HEADER: &str = "Percentage";

    let mut title_width = TITLE_HEADER.len();
    let mut percentage_width = PERCENTAGE_HEADER.len();

    for (title, percentage) in personalities.iter() {
        title_width = title_width.max(title.chars().count());

        let percentage_str = percentage.to_string();
        percentage_width = percentage_width.max(percentage_str.len());
    }

    print_border(title_width, percentage_width);
    print_header_row(
        TITLE_HEADER,
        PERCENTAGE_HEADER,
        title_width,
        percentage_width,
    );
    print_border(title_width, percentage_width);

    for (title, percentage) in personalities.iter() {
        println!(
            "| {title}{} | {percentage}%{} |",
            " ".repeat(title_width - title.chars().count()),
            " ".repeat(percentage_width - percentage.to_string().len() - 1)
        )
    }

    print_border(title_width, percentage_width)
}

fn print_border(title_width: usize, percentage_width: usize) {
    println!(
        "+{}+{}+",
        "-".repeat(title_width + 2),
        "-".repeat(percentage_width + 2)
    );
}

fn print_header_row(title: &str, percentage: &str, title_width: usize, percentage_width: usize) {
    let title_padding = title_width + 2 - title.len();
    let title_left = title_padding / 2;
    let title_right = title_padding - title_left;

    let percentage_padding = percentage_width + 2 - percentage.len();
    let percentage_left = percentage_padding / 2;
    let percentage_right = percentage_padding - percentage_left;

    println!(
        "|{}{title}{}|{}{percentage}{}|",
        " ".repeat(title_left),
        " ".repeat(title_right),
        " ".repeat(percentage_left),
        " ".repeat(percentage_right),
    );
}
