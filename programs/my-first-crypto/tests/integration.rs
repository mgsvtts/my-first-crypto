use anchor_lang::{system_program, Id};
use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use anchor_spl::token_2022::Token2022;
use solana_program_test::ProgramTestContext;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    account::Account, instruction::Instruction, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    signature::Keypair, signer::Signer, transaction::Transaction,
};

#[tokio::test]
async fn my_test() {
    let TestSetUp {
        validator,
        user,
        mint,
        user_token,
        token_pool,
        stacker_pda,
    } = TestSetUp::new();

    let mut context = validator.start_with_context().await;

    let stack_ix = Instruction {
        program_id: my_first_crypto::ID,
        accounts: my_first_crypto::accounts::Stack {
            user: user.pubkey(),
            user_pool_account: stacker_pda,
            mint: mint.pubkey(),
            user_token_account: user_token.pubkey(),
            token_pool_account: token_pool.pubkey(),
            token_program: Token2022::id(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: my_first_crypto::instruction::Stack {
            amount: 1,
            rate: 25,
        }
        .data(),
    };

    let stack_ix = Transaction::new_signed_with_payer(
        &[stack_ix],
        Some(&user.pubkey()),
        &[&user],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(stack_ix)
        .await
        .unwrap();

    let stacker: my_first_crypto::UserPoolAccount =
        load_and_deserialize(context, stacker_pda).await;
}

struct TestSetUp {
    validator: ProgramTest,
    user: Keypair,
    mint: Keypair,
    user_token: Keypair,
    token_pool: Keypair,
    stacker_pda: Pubkey,
}

impl TestSetUp {
    pub fn new() -> Self {
        let mut validator = ProgramTest::new("my-first-crypto", my_first_crypto::ID, None);

        let user = Keypair::new();
        let mint = Keypair::new();
        let user_token = Keypair::new();
        let token_pool = Keypair::new();

        validator.add_account(
            user.pubkey(),
            Account {
                lamports: 10 * LAMPORTS_PER_SOL,
                ..Account::default()
            },
        );
        validator.add_account(mint.pubkey(), Account::default());
        validator.add_account(user_token.pubkey(), Account::default());
        validator.add_account(token_pool.pubkey(), Account::default());

        let (pda, _) = Pubkey::find_program_address(
            &[b"user_pool", user.pubkey().as_ref()],
            &my_first_crypto::ID,
        );

        Self {
            validator,
            user,
            mint,
            user_token,
            token_pool,
            stacker_pda: pda,
        }
    }
}

pub async fn load_and_deserialize<T: AccountDeserialize>(
    mut ctx: ProgramTestContext,
    address: Pubkey,
) -> T {
    let account = ctx
        .banks_client
        .get_account(address)
        .await
        .unwrap() //unwraps the Result into an Option<Account>
        .unwrap(); //unwraps the Option<Account> into an Account

    T::try_deserialize(&mut account.data.as_slice()).unwrap()
}
