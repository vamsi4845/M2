#![no_main]


use risc0_zkvm::guest::env;
use move_vm_integration_tests::tests::public_tests::mutated_accounts;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    
    // run the mutated accounts test
    mutated_accounts(); 

}
