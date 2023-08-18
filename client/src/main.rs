use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;
use zeke_contract as zc;
use zc::client::{
    create_pda, get_shop_obj, get_program,
    print_program_info, refund_to_buyer, run_balance_checks,
    save_new_purchase_data, send_instruction
};
use zc::utils::{
    get_args, get_devnet_2_user, pp, pda_key
};

fn main() {
    let args = get_args();

    // 1. Connect to chain and run balance checks
    let connection = zc::client::establish_connection();
    let user = zc::utils::get_user().unwrap();
    run_balance_checks(&user, &connection);

    let program_keypair = &args[1];
    let program = get_program(program_keypair, &connection).unwrap();
    let pda = pda_key(&user.pubkey(), &program.pubkey()).unwrap();

    // 2. Optional - Create account for program to write its data 
    // (Fee: 5000) (a new acc/addr for a given user and program combination)
    println!("\n2. Create account for program to read/write its data...");
    let res = create_pda(&user, &program, &connection).unwrap();
    println!("--- res : {:?}", res);

    // 3. Print some info
    print_program_info(&user, &connection, &program);

    println!("Escrow info:");
    // Todo: Use FAUCET_PDA as buyer ?
    // use std::str::FromStr;
    // use solana_program::pubkey::Pubkey;
    // const FAUCET_PDA: &str = "4uHT4GBdZPimek4Zv2PquUtup7J8wNkwuU9Lna2pS6zQ";
    // let buyer = Pubkey::from_str(FAUCET_PDA).unwrap();
    // let buyer_keypair = &user;
    //
    let buyer_keypair = get_devnet_2_user().unwrap();
    let buyer = buyer_keypair.pubkey();
    let seller = user.pubkey();
    println!("--- buyer : {}", &buyer);
    println!("--- seller: {}", &seller);
    let pda_balance_0 = connection.get_balance(&pda).unwrap();
    let buyer_balance_0 = connection.get_balance(&buyer).unwrap();
    println!("--- PDA bal  : {}", pp(pda_balance_0));
    println!("--- buyer bal: {}", pp(buyer_balance_0));
    // println!("--- seller bal: {}", pp(connection.get_balance(&seller).unwrap()));        

    // 4. write
    if args[2] != "w" {
        println!("\nEND\nProgram ended without write to chain\n");
        return
    }
    const TEN_LAMPORTS: u64 = 10;
    println!("\n4. Write to chain: Sending transaction(s) ...");

    // a. Buyer sends lamports to Escrow's PDA
    println!("\na. Buyer sending {} lamports to Escrow's PDA ...", TEN_LAMPORTS);
    let ins = transfer(&buyer, &pda, TEN_LAMPORTS);
    let res = send_instruction(ins, &buyer_keypair, &buyer, &connection);
    println!("res: {:?}", res);
    let pda_balance_1 = connection.get_balance(&pda).unwrap();
    let buyer_balance_1 = connection.get_balance(&buyer).unwrap();
    println!("--- PDA bal  : {}", pp(pda_balance_1));
    const TX_COST: u64 = 5000; // todo: get this from chain
    println!("--- buyer bal: {} (tx fee: {})", 
        pp(buyer_balance_1), pp(TX_COST)
    );
    assert_eq!(pda_balance_1, pda_balance_0 + TEN_LAMPORTS);
    assert_eq!(buyer_balance_1, buyer_balance_0 - TEN_LAMPORTS - TX_COST);

    // b. save_new_purchase_data
    let seller = buyer.clone(); // For now, todo
    println!("\nb. Saving new purchase data ...");
    println!(
        "> before: {:#?}",
        get_shop_obj(&user, &program, &connection).unwrap()
    );
    let res = save_new_purchase_data(
        &user, &program, &connection,
        buyer, TEN_LAMPORTS as u8, seller
    );
    println!("res: {:?}", res);
    let purchase_data = get_shop_obj(&user, &program, &connection).unwrap();
    println!("> after: {:#?}", purchase_data);
    assert_eq!(purchase_data.paid_amount, TEN_LAMPORTS as u8);
    assert_eq!(purchase_data.buyer, buyer);

    // c. refund_to_buyer
    println!("\nc. Refunding {} to buyer ...", TEN_LAMPORTS);
    let res = refund_to_buyer(&user, &program, &connection, buyer);
    println!("res: {:?}", res);
    let pda_balance_2 = connection.get_balance(&pda).unwrap();
    let buyer_balance_2 = connection.get_balance(&buyer).unwrap();
    println!("--- PDA bal  : {}", pp(pda_balance_2));
    println!("--- buyer bal: {}", pp(buyer_balance_2));
    assert_eq!(pda_balance_2, pda_balance_0);
    assert_eq!(buyer_balance_2, buyer_balance_0 - TX_COST);

    println!("\nEnd\n");
}
