pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let rows = garden.len();
    let cols = garden[0].len();

    let garden_bytes: Vec<&[u8]> = garden.iter().map(|row| row.as_bytes()).collect();

    let mut result = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut new_row = String::with_capacity(cols);

        for c in 0..cols {
            let cell = garden_bytes[r][c];
            if cell == b'*' {
                new_row.push('*');
            } else {
                let mut count = 0;
                for dr in [-1i32, 0, 1] {
                    for dc in [-1i32, 0, 1] {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;

                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            if garden_bytes[nr as usize][nc as usize] == b'*' {
                                count += 1;
                            }
                        }
                    }
                }

                if count > 0 {
                    new_row.push((b'0' + count as u8) as char);
                } else {
                    new_row.push(' ');
                }
            }
        }

        result.push(new_row);
    }

    result
}