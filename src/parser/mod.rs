/// contains all the parsing structures of ghvm
mod ast;
use ast::{Module, Statement};

peg_file! grammar("grammar.rustpeg");

#[cfg(test)]
mod tests;

pub fn parse(body: &str) -> Result<ast::Module, String> {
    try!(grammar::module(&processed_body));
}
