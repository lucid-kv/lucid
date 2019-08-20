/**
 * Description: High performance and distributed KV ledger.
 * Author: by Clint.Network
 * Date: 28/07/2019
*/

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

mod lucid;
use lucid::Lucid;

fn main() -> Result<(), String> {
    let mut lucid = Lucid::new();
    lucid.default()
}
