use std::io::Read;


#[allow(unused_variables)]
fn read_version(transaction_bytes: &mut &[u8])->u32 {
    let mut buffer = [0;4];
    transaction_bytes.read(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)

    
}

#[allow(unused_variables)]
fn main() {
    let transaction_hex  ="02000000000101b0e1f89573c0d5313848c962b4af6df0ce6683776f88dce5a8052847aea05aac0000000000fdffffff0273170d8f00000000160014bb6380980fca48fce5d5a582452ebe9cd1396da600e1f505000000001600146a5bd823854e872619f226a5fbe81ae6d4c1435f02473044022038260a253503ce5d74a941076d6399544d04a6dfdcf9249f07a090811d7d7c4b022064e1befcd5529acc0e1a3f89d4bdf33db1b60d33b0dadd41435453c180ebdf28012102d514ac4d4be3af4525b1e4742f8d5eb0b0400366714c72ef831f0ac9722c2de92f010000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut byte_slice = transaction_bytes.as_slice();
    let version = read_version(&mut byte_slice);

    println!("version: {}!", version);
}
