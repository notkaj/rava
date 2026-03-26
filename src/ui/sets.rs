use tui_barchart_ext::bar_symbols::Set;

#[allow(dead_code)]
pub const THREE_LINES: Set = set_from_symbol("☰");

#[allow(dead_code)]
pub const SIX_LINES: Set = set_from_symbol("䷀");

#[allow(dead_code)]
pub const CHECKERS: Set = set_from_symbol("🙾");

#[allow(dead_code)]
pub const MORE_CHECKERS: Set = set_from_symbol("🮕");

#[allow(dead_code)]
pub const ROUNDED_SQUARES: Set = set_from_symbol("▢");

pub const fn set_from_symbol(symbol: &'static str) -> Set<'static> {
    Set {
        full: symbol,
        seven_eighths: symbol,
        three_quarters: symbol,
        five_eighths: symbol,
        half: " ",
        three_eighths: " ",
        one_quarter: " ",
        one_eighth: " ",
        empty: " ",
    }
}
