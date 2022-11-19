fn main(){
    use comfy_table::{Table, Row};
    let mut table = Table::new();
    let rows = vec![
    Row::from(vec!["One", "Two"]),
    Row::from(vec!["Three", "Four"])
    ];
    table.add_rows(rows);

    println!("{table}");
}