use rkyv::{api::high::to_bytes_in, ser::writer::Buffer};

fn main() -> eyre::Result<()> {
    let mut test = [0u8; 16];
    let a = Buffer::from(&mut test);

    let _ = to_bytes_in::<_, rkyv::rancor::Error>(&vec![9u8], a);

    println!("{:?}", test);
    Ok(())
}
