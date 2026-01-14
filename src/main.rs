use {
    agave_validator::admin_rpc_service,
    std::Path
}
fn main() {
    let ledger_path=Path::new("/mnt/ledger");
    let admin_client= match admin_rpc_service::connect(ledger_path){
        Ok(client)=>client,
        Err(_)=>{
            println!("Failed to connect to admin RPC service");
            return;
        }   
    }
    println!("Hello, world!");
}
