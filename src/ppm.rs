use crate::canvas::*;

pub fn canvas_to_ppm(canvas: Canvas) -> String {
    let header = format!(
        "P3
{} {}
255
",
        canvas.width, canvas.height
    );

    let mut data = String::new();
    for y in 0..canvas.height {
        let row: Vec<_> = canvas.data[3 * (y * canvas.width)..3 * ((y + 1) * canvas.width)]
            .iter()
            .map(|c| c.to_string())
            .collect();

        // PPM files need to be wrapped to 70 chars.
        let mut chars_written = 0;
        for c in row {
            let len = 1 + c.len();
            if len + chars_written > 70 {
                data.push_str("\n");
                chars_written = 0;
            }

            if chars_written == 0 {
                data.push_str(&c);
                chars_written += len - 1;
            } else {
                data.push_str(" ");
                data.push_str(&c);
                chars_written += len;
            }
        }

        data.push_str("\n");
    }

    header + &data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::*;

    #[test]
    fn constructing_the_ppm_header() {
        let c = canvas(5, 3);
        let ppm = canvas_to_ppm(c);
        assert!(ppm.starts_with(
            "P3
5 3
255
"
        ));
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);

        c.set_color(0, 0, &c1);
        c.set_color(2, 1, &c2);
        c.set_color(4, 2, &c3);

        let ppm = canvas_to_ppm(c);
        assert!(ppm.ends_with(
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"
        ))
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = canvas(10, 2);

        for x in 0..c.width {
            for y in 0..c.height {
                c.set_color(x, y, &color(1.0, 0.8, 0.6));
            }
        }

        let ppm = canvas_to_ppm(c);
        println!("{}", ppm);
        assert!(ppm.ends_with(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"
        ))
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline() {
        let c = canvas(5, 3);
        let ppm = canvas_to_ppm(c);
        assert!(ppm.ends_with("\n"));
    }
}
