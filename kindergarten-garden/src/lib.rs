pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    const STUDENTS: [&str; 12] = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    fn plant_name(c: char) -> &'static str {
        match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => panic!("Unknown plant code: {}", c),
        }
    }

    let idx = STUDENTS
        .iter()
        .position(|&name| name == student)
        .expect("Unknown student");

    let mut rows = diagram.lines();
    let row1 = rows.next().expect("Missing first row");
    let row2 = rows.next().expect("Missing second row");

    let start = idx * 2;
    let cups = [
        row1.chars().nth(start).expect("Row 1 too short"),
        row1.chars().nth(start + 1).expect("Row 1 too short"),
        row2.chars().nth(start).expect("Row 2 too short"),
        row2.chars().nth(start + 1).expect("Row 2 too short"),
    ];

    cups.into_iter().map(plant_name).collect()
}