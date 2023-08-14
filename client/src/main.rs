use solana_sdk::signer::Signer;
use solana_program::pubkey::Pubkey;
use solana_program::native_token::lamports_to_sol;
use std::str::FromStr;
use zeke_contract as zc;
use zc::utils::{
    program_derived_account_key, 
    seed_for_program_derived_account_creation
};

fn main() {
    let pretty_print = |num: u64| { // e.g. 10000 -> 10_000
        num.to_string().as_bytes().rchunks(3).rev().map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>().unwrap().join("_")  // separator
    };
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        eprintln!(
            "\nError: Wrong number of args.
            usage: e.g. \
            cargo r ../program/target/deploy/helloworld-keypair.json r shop1
            (w: write, r: read)
            ",
        );
        std::process::exit(-1);
    }
    let keypair_path = &args[1];

    // 1. Connect to chain
    let connection = zc::client::establish_connection().unwrap();
    println!(
        "\n1. Connected to remote solana node running version ({}).\n",
        connection.get_version().unwrap()
    );

    let balance_requirement = zc::client::get_balance_requirement(&connection).unwrap();
    println!(
        "({}) lamports are required for this transaction.",
        pretty_print(balance_requirement)
    );

    let user = zc::utils::get_user().unwrap();
    let user_balance = zc::client::get_user_balance(&user, &connection).unwrap();
    println!("User: {:?}",user.pubkey());
    println!("Balance: {} Sol ({} lamports)", 
        lamports_to_sol(user_balance), pretty_print(user_balance)
    );
    // println!("User {:?}: {} lamports",user.pubkey(), user_balance);

    if user_balance < balance_requirement {
        let request = balance_requirement - user_balance;
        println!(
            "User does not own sufficent lamports. Airdropping ({}) lamports.",
            request
        );
        zc::client::request_airdrop(&user, &connection, request).unwrap();
    }

    let program = zc::client::get_program(keypair_path, &connection).unwrap();

    // 2. Optional - Create account for program to write its data 
    // (Fee: 5000) (a new addr for a given user and program combination)
    println!("\n2. Create account for program to read/write its data...");
    zc::client::create_program_derived_account(&user, &program, &connection).unwrap();

    // Print some info
    println!("\nProgram: {:?}", program.pubkey());
    let pda = program_derived_account_key(&user.pubkey(), &program.pubkey()).unwrap();
    println!("Program's data account to read/write: {:?}", pda);
    println!("(derived addr for a given user and program combination)\n");

    println!("--- PDA name: {}", seed_for_program_derived_account_creation());
    println!("--- PDA bal: {}", connection.get_balance(&pda).unwrap());

    //
    let buyer = user.pubkey();
    let seller = user.pubkey();
    println!("--- buyer: {}: {}", &buyer, connection.get_balance(&buyer).unwrap());
    println!("--- seller  : {}: {}", &seller, connection.get_balance(&seller).unwrap());
    // 3. write
    if args[2] == "w" {
        println!("\n3. Write to chain: Sending tx");
        // x
        let _ = zc::client::refund_to_buyer(&user, &program, &connection, buyer);
        // x
        // println!("Quick read before write:");
        // println!(
        //     "> Shop obj: {:#?}",
        //     zc::client::get_shop_obj(&user, &program, &connection).unwrap()
        // );
        // let _ = zc::client::save_new_purchase_data(
        //     &user, &program, &connection,
        //     buyer, 200, seller
        // );
        // x
        // let from = pda;
        // let to = Pubkey::from_str("4roTv8dUHJrybx5goVLvwmewKWgMzo5h4dHPM8EcjydM").unwrap();
        // println!("--- from: {}: {}", &from, connection.get_balance(&from).unwrap());
        // println!("--- to  : {}: {}", &to, connection.get_balance(&to).unwrap());
        // let _ = zc::client::send_lamports(
        //     &user, &program, &connection,
        //     from, to, 5
        // );
    } else { 
        println!("\n3. Skipping \"Write to chain\"");
    }

    // 4. read
    println!("\n4. Read from chain:");
    println!(
        "> Shop obj: {:#?}",
        zc::client::get_shop_obj(&user, &program, &connection).unwrap()
    );
    println!("--- PDA bal: {}", connection.get_balance(&pda).unwrap());
    println!("--- buyer: {}: {}", &buyer, connection.get_balance(&buyer).unwrap());
    println!("--- seller  : {}: {}", &seller, connection.get_balance(&seller).unwrap());

    println!("\nEnd\n");
}
