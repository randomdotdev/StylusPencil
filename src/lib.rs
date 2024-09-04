#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{alloy_primitives::Address, msg, prelude::*, storage::StorageAddress};

mod erc20;
use erc20::*;

const SUPPLY: &str = "1000000000";
const OWNER: &str = "0xDf09D3fedFEd7C8e2eD91C6FCD2262e30e193F30";

struct StylusPencilParams;
impl Erc20Params for StylusPencilParams {
    const NAME: &'static str = "Stylus Pencil";
    const SYMBOL: &'static str = "PEN";
    const DECIMALS: u8 = 18;
}

sol_storage! {
    #[entrypoint]
    struct StylusPencil {
        #[borrow]
        Erc20<StylusPencilParams> erc20;
        StorageAddress owner;
        bool initialized;
    }
}

/*
* StylusPencil is a simple ERC20 token contract written in Rust.
* @author https://x.com/randomdotdev
*/
#[public]
#[inherit(Erc20<StylusPencilParams>)]
impl StylusPencil {
    pub fn intialize(&mut self) -> Result<(), Erc20Error> {
        if self.initialized.get() {
            panic!("already initialized");
        }
        self.initialized.set(true);

        let owner_address = Address::parse_checksummed(OWNER, None).expect("Invalid address");
        if msg::sender() != owner_address {
            panic!("only owner can initialize");
        }
        self.owner.set(Address::ZERO);

        let supply = alloy_primitives::utils::parse_ether(SUPPLY).unwrap();
        self.erc20.mint(msg::sender(), supply)?;
        Ok(())
    }

    pub fn owner(&self) -> Address {
        self.owner.get()
    }
}
