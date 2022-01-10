## Performance improvements
- Mutable references to self instead of new instances for Vector/Point etc?
- Canvas implementation
- Avoid allocations (String::with_capacity when possible, and check for reallocations!)

- Exemple of multiple iterations until we got an acceptable algo :
```
First try (ugly but working, a bit slow):
    fn split_ppm_lines_too_long(pixel_data: &str) -> String {
        /* The final PPM pixel_data in which we split lines greater than 70 chars will be the same length as the pixel_data, since we are only
         * replacing spaces by newlines. */
        let mut split_pixel_data = String::with_capacity(pixel_data.len());
    
        let lines: Vec<&str> = pixel_data.split('\n').collect();
        let line_count = lines.len();
    
        for (line_index, line) in lines.into_iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                // Insert a newline if we arrive at a char which position is a multiple of 70.
                if (i > 0) && (i % PPM_MAX_CHARACTERS_PER_LINE == 0) {
                   let mut j = i;
                    // To avoid splitting a number (pixel), we go back to the white space before it to insert a new line.
                    while pixel_data.chars().nth(j).unwrap().is_numeric() {
                        split_pixel_data.pop();
                        j -= 1;
                    }
                    // When we have found a whitespace, we insert a new line.
                    split_pixel_data.push('\n');
                    // Then, we insert what was after the white space (the one before the split number) and until the current iterated char (included).
                    split_pixel_data.push_str(&pixel_data[(j + 1)..=i]);
                } else {
                    split_pixel_data.push(c);
                }
            }
            // Insert a new line unless we've arrived at the last line.
            if line_index < (line_count - 1) {
                split_pixel_data.push('\n');
            }
        }
    
        split_pixel_data
    } 

Second try (less ugly but still difficult to read, might be faster than the first):
    fn split_ppm_lines_too_long(pixel_data: &str) -> String {
        for (line_index, line) in lines.into_iter().enumerate() {
            let colors: Vec<&str> = line.split(' ').collect();
            let color_count = colors.len();
    
            for (color_index, color) in colors.iter().enumerate() {
                let last_line_index = lines.len() - 1;
                let current_line_length = lines[last_line_index].len();
    
                // If the current line would be greater than 70 char when appended with the next color, insert a new line and a space
                if (color.len() + current_line_length) > PPM_MAX_CHARACTERS_PER_LINE {
                    lines.push(color.to_string());
                    // For safety only
                    assert!(color.len() < PPM_MAX_CHARACTERS_PER_LINE);
                    lines[last_line_index + 1].push(' ');
                    // Else append current line
                } else {
                    lines[last_line_index].push_str(color);
                    let can_insert_space =
                        (current_line_length + color.len() + 1) < PPM_MAX_CHARACTERS_PER_LINE;
                    let is_last_color = color_index == (color_count - 1);
    
                    let next_color_result = colors.get(color_index + 1);
                    if let Some(next_color) = next_color_result {
                        let can_insert_next_color =
                            (current_line_length + color.len() + 1 + next_color.len())
                                < PPM_MAX_CHARACTERS_PER_LINE;
                        if can_insert_space && !is_last_color && can_insert_next_color {
                            lines[last_line_index].push(' ');
                        } else {
                            lines.push(String::new());
                        }
                    }
                }
            }
            if line_index < line_count - 1 {
                lines.push(String::new());
            }
        }
    }


Third try: More idiomatic and much faster than the previous ones
(using iterators and peek()), more readable)
Though could still be improved by avoiding creating a Vec<String> and then mapping it to a final String
    fn split_ppm_lines_too_long(pixel_data: &str) -> String {

        // Create a vec with 1 string that will contain the split lines
        let mut lines: Vec<String> = vec![String::new()];

        let mut it_lines = pixel_data.split('\n').peekable();
        while let Some(line) = it_lines.next() {

            let mut it_colors = line.split(' ').peekable();
            while let Some(color) = it_colors.next() {
                let last_line_index = lines.len() - 1;
                let current_line_length = lines[last_line_index].len();

                // If the current line would be greater than 70 char when appended with the next color, insert the color in a new line.
                if (color.len() + current_line_length) > PPM_MAX_CHARACTERS_PER_LINE {
                    lines.push(color.to_string());
                // Else append the current line
                } else {
                    lines[last_line_index].push_str(color);
                    if let Some(next_color) = it_colors.peek() {
                        let can_insert_next_color_into_line =
                            (current_line_length + color.len() + 1 + next_color.len())
                                < PPM_MAX_CHARACTERS_PER_LINE;
                        if can_insert_next_color_into_line {
                            lines[last_line_index].push(' ');
                        } else {
                            lines.push(String::new());
                        }
                    }
                }
            }

            // If there is a next line, insert a new line in the final string
            if it_lines.peek().is_some() {
                lines.push(String::new());
            }
        }
    }
```
