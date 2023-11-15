use revm::{
    primitives::{address, AccountInfo, TxEnv, Address, U256, Bytes, Bytecode, Output},
    InMemoryDB, EVM,
};

use ethers_contract::{BaseContract, Lazy};
use ethers_core::abi::{parse_abi};
use std::{fs::File, io::Write, path::Path, fs};
use std::str::FromStr;


fn main() -> eyre::Result<()> {
    simulate()?;
    Ok(())
}

// to compile: solc examples/test.sol --bin
pub static CONTRACT_CODE: Lazy<Bytes> = Lazy::new(|| {
    "608060405234801561000f575f80fd5b5060b28061001c5f395ff3fe6080604052348015600e575f80fd5b50600436106026575f3560e01c8063f8a8fd6d14602a575b5f80fd5b60306044565b604051603b91906065565b60405180910390f35b5f63deadbeef905090565b5f819050919050565b605f81604f565b82525050565b5f60208201905060765f8301846058565b9291505056fea26469706673582212200e680778233a75ecc01a5921dd6733f3834c4db8098ad8cfcda1cd4a98c3cb0864736f6c63430008170033"
	.parse().unwrap()});

fn simulate() -> eyre::Result<()> {
    let mut db = InMemoryDB::default();

    // Populate the DB pre-state,
    let addrA = address!("4838b106fce9647bdf1e7877bf73ce8b0bad5f97");    
    {
	let info = AccountInfo {
	    balance: U256::from(420),
            ..Default::default()
	};
	db.insert_account_info(addrA, info);
    }

    let addrB = address!("F2d01Ee818509a9540d8324a5bA52329af27D19E");    
    {
	let info = AccountInfo {
	    balance: U256::from(12),
	    nonce: 1,
	    code: Some(Bytecode::new_raw((*CONTRACT_CODE.0).into())),
            ..Default::default()
	};
	db.insert_account_info(addrB, info);
    }

    // Setup the EVM with the configured DB
    // The EVM will ONLY be able to access the witnessed state, and
    // any simulation that tries to use state outside of the provided data
    // will fail.
    let mut evm = EVM::new();
    evm.database(db);

    let abi = BaseContract::from(parse_abi(&[
        "function test() public returns (uint)",
    ])?);
    let calldata = abi.encode("test", ())?;

    evm.env.tx = TxEnv {
        caller: addrA,
        transact_to: revm::primitives::TransactTo::Call(addrB),
	//value: U256::from(120),
	data: revm::primitives::Bytes::from(calldata.0.clone()),
        ..Default::default()
    };

    let result = evm.transact_ref()?;
    //let outp1 = result.output().unwrap();
    //let outp = match outp1 {
    //    Output::Call(o) => o
    //};
    //let out = abi.decode_output("test", outp1)?;
    //dbg!(out);
    dbg!(&result);

    Ok(())
}
