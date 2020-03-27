use crate::symbol_table::SymbolTable;
use crate::to_bytecode::ToBytecode;

pub fn to_machine_code<'a, T: ToBytecode>(
    symbols: &mut SymbolTable,
    tokens: &Vec<T>
) -> String {
    tokens
        .into_iter()
        .map(|symbol| symbol.to_bytecode(symbols))
        .fold("".to_string(), |acc, t| match t {
            Some(str) => format!("{}\n{}", acc, str),
            None => acc
        })
        .trim_start_matches("\n")
        .to_string()
}
