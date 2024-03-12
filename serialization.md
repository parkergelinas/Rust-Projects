```toml
[dependencies]
borsh = "0.9"
borsh-derive = "0.9"
solana-program = "1.8"
```

```
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MyData {
    pub number: u32,
    pub data: Vec<u8>,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let input_data = MyData::try_from_slice(input)?;

    // input_data logic here

    Ok(())
}
```
