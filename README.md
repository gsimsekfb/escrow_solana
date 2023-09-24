# Escrow program (smart contract) 
(work in progress) 

This repository demonstrates how to create/deploy and use a simple Escrow program (smart contract) for the purchase of tokens and physical items on the Solana blockchain.

Features: 
- IsPostDelivered (read from Oracle, this is for supporting the purchase of physical items)
- TransferTokenToBuyer (WIP)
- RefundToBuyer
- SavePurchaseData 

E.g.   
Let's have a quick look an example user story from program deployment to program usage:  

Story-2: Refund to buyer (an example unsuccessful purchase)
- Buyer sends X SOL to Escrow 
- Client code triggers Escrow program's feature: save purchase data 
- Client code triggers Escrow program's feature: refund to buyer    

```
// A. Deploy program (smart contract)
cd program
cargo build-bpf
solana program deploy target/deploy/escrow.so 
Program Id: Es5dTX5VbmPfE7NVBD6hozEC6M77NCHNVtZ1BGMdU7M6
Fee: 10000
e.g. TX:
https://explorer.solana.com/tx/5YabTWTQcj6do8GhDqcc3XFe3YhRMFQWZCf8amtjLMAf7e1qzPs8pJi55xYJ91fbzrvBBiwNAEMpWeciGw9UEknN?cluster=devnet

// B. Use program (smart contract)
cd client

//// Run Story-2: Refund to buyer
//// - Buyer sends X SOL to Escrow
//// - Escrow ACTION::SavePurchaseData
//// - Escrow ACTION::RefundToBuyer

cargo r ../program/target/deploy/escrow-keypair.json w buyer3

1. Connected to remote solana node running version (1.16.11).

(1_662_320) lamports are required for this transaction.

2. Create account for program to read/write its data...
--- Program's object size: 39 bytes
--- min_balance_for_rent_exemption: 1_162_320
... not created, account may already exist 
--- res : ()

3. Info
User   : 7GDXzkmtqNG2BZmesUyv2qrbRoovv71TApd1bWSsZAuc
Balance: 23.841721951 Sol (23_841_721_951 lamports)
Program: 6HJLw99sjqgpv8DPizbCRUPvsnCUcQ6zN1YEmXio35xr
PDA    : AYysd2G9tP8eE6hLXsd5tPTKCcra4rJDHFsQDphHGvfq
  (aka Program's data account to read/write)
  (aka Derived addr for a given user and program combination)
PDA name: buyer3

Escrow info:
--- buyer : 7HZFjjPgQAMRiyWyNgCB1A8q7s6FijPGidqgkte41oUq
--- seller: 7GDXzkmtqNG2BZmesUyv2qrbRoovv71TApd1bWSsZAuc
--- PDA bal  : 1_162_320
--- buyer bal: 2_999_929_920

4. Write to chain: Sending transaction(s) ...

a. Buyer sending 10 lamports to Escrow's PDA ...
res: Ok(())
--- PDA bal  : 1_162_330
--- buyer bal: 2_999_924_910 (tx fee: 5_000)

b. Saving new purchase data ...
> before: EscrowSchema {
    buyer: 7HZFjjPgQAMRiyWyNgCB1A8q7s6FijPGidqgkte41oUq,
    paid_amount: 10,
    refunded: true,
    post_delivered: false,
    eth_usd_price: 1637,
}
res: Ok(())
> after: EscrowSchema {
    buyer: 7HZFjjPgQAMRiyWyNgCB1A8q7s6FijPGidqgkte41oUq,
    paid_amount: 10,
    refunded: false,
    post_delivered: false,
    eth_usd_price: 1637,
}

c. Refunding 10 to buyer ...
--- refund_to_buyer() 7HZFjjPgQAMRiyWyNgCB1A8q7s6FijPGidqgkte41oUq ...
res: Ok(())
--- PDA bal  : 1_162_320
--- buyer bal: 2_999_924_920

Purchase complete:
EscrowSchema {
    buyer: 7HZFjjPgQAMRiyWyNgCB1A8q7s6FijPGidqgkte41oUq,
    paid_amount: 10,
    refunded: true,
    post_delivered: false,
    eth_usd_price: 1637,
}

End


```
