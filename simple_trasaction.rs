use  std::time::Duration;

user ethers::{
    prelude::{Address, Localwallet ,Middleware, Privider, Signer, TransactionRequest, U256 },
    utils::Ganache,
};
use  eyre::{ContexCompat, Result};
use hex::Tohex;

#[tokio::main]
async fn main() -> Result<()>{
    let  mnemonic = "zxcvbnm asdfghjkl poiuytrewq";
    let  ganache = Ganache::new().mnemonic(mnemonic).spawn(); 
    println!("HTTP Endpoint:{}" , ganache.Endpoint());

    let  wallet : Localwallet = ganache.Keys()[0].clone().into();

    println!("wallet first address : {}" , first_address.encode_hex::<String>()  );
// comment 

    let  provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10) );
    
    let  first_balance = provider.get_balance(first_address, None).await?;

    println!("wallet first address balance : {} " , first_balance);

    let other_address_hex = "0x591d66b2c7efb2e1d709f4d517b8f24f8562a9f177517f7b17f5cf944c2e2faa";
    let  other_address = "0x3aE730c7eE6bC5B00Bd67aDb4F4c020d7dA539f1".parse::<Address>()?;
    let other balance = provider.get_balance(other_address, None ).

    println!("wallet address {} ,  {} " , other_address_hex , other_address );

    //comment 
    let  tx  = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);
    
    let receipt = provider 
        .send_transaction(tx, None);
        .await?
        .log_msg("pending transfer")
        .confirmations(1)
        .await?
        .context("Missing receipt");

    println!(
        "TX mined in block  {} ",
        receipt.block_number.context("cannot get block number")?

    );
    println!(
        "balance of {}  {}" ,
        other_address_hex ,
        provider.get_balance(other_address . None).await?
    );
    ok();




}