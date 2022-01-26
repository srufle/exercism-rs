#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    } else if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if number.len() == 0 {
        return Ok(vec![0]);
    }
    let most_sig_dig = number[0];
    let mut acc = if number.len() > 1 {
        most_sig_dig * from_base
    } else {
        most_sig_dig
    };

    for index in 1..number.len() {
        let n = number[index];
        if n >= from_base {
            return Err(Error::InvalidDigit(n));
        }
        acc = acc + number[index];

        if index < number.len() - 1 {
            acc = acc * from_base;
        }
    }

    let ans = number_to_digits(acc, to_base);

    Ok(ans)
}

// How to split a number into a vec of digits
// https://stackoverflow.com/questions/41536479/how-do-i-split-an-integer-into-individual-digits
fn number_to_digits(number: u32, base: u32) -> Vec<u32> {
    fn x_inner(number: u32, base: u32, xs: &mut Vec<u32>) {
        if number >= base {
            x_inner(number / base, base, xs);
        }
        xs.push(number % base);
    }
    let mut xs = Vec::new();
    // Recursively divide input number by base,
    // while number is greater or equal to base
    x_inner(number, base, &mut xs);
    xs
}
