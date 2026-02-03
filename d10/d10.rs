use aoc2016::grid::Grid;

fn main() {
    let input = r#"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
        "#;
    let grid = Grid::from_lines(input.lines(), |_, c| c as u8 - b'0');
    println!("{}", grid);
}
