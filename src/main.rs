/**
 * Description: High performance and distributed KV ledger.
 * Author: by Clint.Network
 * Date: 28/07/2019
*/

#[macro_use] extern crate nickel;

mod lucid;
use lucid::Lucid;

fn main()
{
    Lucid::init()
        .banner()
        .commandline();
}