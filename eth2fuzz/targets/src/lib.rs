extern crate ssz;

use ssz::Decode;

use types::{
    Attestation, AttesterSlashing, BeaconBlock, BeaconState, Deposit, MainnetEthSpec,
    ProposerSlashing, SignedBeaconBlock, SignedVoluntaryExit,
};


mod attestation;
#[inline(always)]
pub fn fuzz_lighthouse_attestation(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {

    // Verify that data is a correct Attestation ssz
    let attestation = match Attestation::from_ssz_bytes(&data) {
        Ok(attestation) => attestation,
        Err(_e) => return,
    };

    let _ = attestation::process_attestation(beaconstate, attestation);
}

mod attester_slashing;
#[inline(always)]
pub fn fuzz_lighthouse_attester_slashing(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {

    let attester_slashing = match AttesterSlashing::from_ssz_bytes(&data) {
        Ok(attester_slashing) => attester_slashing,
        Err(_e) => return,
    };

    let _ = attester_slashing::process_attester_slashing(beaconstate, attester_slashing);
}

mod block;
#[inline(always)]
pub fn fuzz_lighthouse_block(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {

    let block = match SignedBeaconBlock::from_ssz_bytes(&data) {
        Ok(block) => block,
        Err(_e) => return,
    };

    let _ = block::state_transition(beaconstate, block, true);
}


mod block_header;
#[inline(always)]
pub fn fuzz_lighthouse_block_header(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {

    let block = match BeaconBlock::from_ssz_bytes(&data) {
        Ok(block) => block,
        Err(_e) => return,
    };

    let _ = block_header::process_header(beaconstate, block);
}

mod deposit;
#[inline(always)]
pub fn fuzz_lighthouse_deposit(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {

    let deposit = match Deposit::from_ssz_bytes(&data) {
        Ok(deposit) => deposit,
        Err(_e) => return,
    };

    let _ = deposit::process_deposit(beaconstate, deposit);
}

mod proposer_slashing;
#[inline(always)]
pub fn fuzz_lighthouse_proposer_slashing(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {

    let proposer_slashing = match ProposerSlashing::from_ssz_bytes(&data) {
        Ok(proposer_slashing) => proposer_slashing,
        Err(_e) => return,
    };

    let _ = proposer_slashing::process_proposer_slashing(beaconstate, proposer_slashing);
}

mod voluntary_exit;
#[inline(always)]
pub fn fuzz_lighthouse_voluntary_exit(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {

    let voluntary_exit = match SignedVoluntaryExit::from_ssz_bytes(&data) {
        Ok(voluntary_exit) => voluntary_exit,
        Err(_e) => return,
    };

    let _ = voluntary_exit::process_voluntary_exit(beaconstate, voluntary_exit);
}

mod beaconstate;
#[inline(always)]
pub fn fuzz_lighthouse_beaconstate(_beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {

    // We are not using the provided beaconstate here

    let mut beaconstate = match BeaconState::from_ssz_bytes(&data) {
        Ok(beaconstate) => beaconstate,
        _ => return,
    };

    beaconstate::fuzz_beaconstate_accessors(&mut beaconstate);
}