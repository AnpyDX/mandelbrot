use num::Complex;

pub struct Arguments {
    pub output: String,
    pub size: (u32, u32),
    pub bounds: (Complex<f64>, Complex<f64>)
}

pub fn parse_cmds(args: Vec<String>) -> Option<Arguments> {
    if args.len() != 4 {
        return None;
    }

    let output = args[0].clone();
    let size: (u32, u32);
    let mut bounds = (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));
    
    match parse_pair(&args[1], "x") {
        Some((width, height)) => size = (width, height),
        _ => return None
    }

    match parse_pair(&args[2], ",") {
        Some((re, im)) => bounds.0 = Complex { re, im },
        _ => return None
    }

    match parse_pair(&args[3], ",") {
        Some((re, im)) => bounds.1 = Complex { re, im },
        _ => return None
    }

    Some(Arguments{ output, size, bounds })
}

fn parse_pair<T: std::str::FromStr>(src: &str, seperator: &str) -> Option<(T, T)> {
    match src.find(seperator) {
        Some(index) => {
            match (T::from_str(&src[0..index]), T::from_str(&src[index + 1..])) {
                (Ok(left), Ok(right)) => return Some((left, right)),
                _ => return None
            }
        },
        _ => return None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pair() {
        let result: Option<(u32, u32)> = parse_pair("1000x2000", "x");
        assert_eq!(result, Some((1000, 2000)));
    }
}