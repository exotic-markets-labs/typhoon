use {
    program_bench::{BenchResult, Bencher},
    sha2::{Digest, Sha256},
    solana_instruction::{AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
    std::path::PathBuf,
};

const IX_NAMES: &[&str] = &[
    "ping",
    "log",
    "create_account",
    "transfer",
    "unchecked_accounts",
];

fn discriminator(name: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{name}"));
    let hash = hasher.finalize();

    let mut discriminator = [0; 8];
    discriminator[..8].copy_from_slice(&hash[..8]);
    discriminator.to_vec()
}

pub fn runner(name: &str) -> BenchResult {
    let mut so_path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benches/programs/target/deploy"
    ));
    so_path.push(format!("{name}.so",));

    let mut bencher = Bencher::new(so_path);

    let program_id = Pubkey::from_str_const("Bench111111111111111111111111111111111111111");

    let data = if name == "star_frame" {
        discriminator("ping")
    } else {
        vec![0]
    };
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![],
            data,
        }],
        Some(&bencher.payer().pubkey()),
        &[bencher.payer()],
        bencher.hash(),
    );
    bencher.execute_tx(IX_NAMES[0], tx);

    let data = if name == "star_frame" {
        discriminator("log")
    } else {
        vec![1]
    };
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![],
            data,
        }],
        Some(&bencher.payer().pubkey()),
        &[bencher.payer()],
        bencher.hash(),
    );
    bencher.execute_tx(IX_NAMES[1], tx);

    let new_account = Keypair::new();
    let account_metas = vec![
        AccountMeta::new(bencher.payer().pubkey(), true),
        AccountMeta::new(new_account.pubkey(), true),
        AccountMeta::new_readonly(solana_system_interface::program::ID, false),
    ];

    let data = if name == "star_frame" {
        discriminator("create_account")
    } else {
        vec![2]
    };
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: account_metas,
            data,
        }],
        Some(&bencher.payer().pubkey()),
        &[bencher.payer(), &new_account],
        bencher.hash(),
    );
    bencher.execute_tx(IX_NAMES[2], tx);

    let data = if name == "star_frame" {
        let mut data = discriminator("transfer");
        data.append(&mut vec![100, 0, 0, 0, 0, 0, 0, 0]);
        data
    } else {
        vec![3, 100, 0, 0, 0, 0, 0, 0, 0]
    };
    let new_account = Keypair::new();
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(bencher.payer().pubkey(), true),
                AccountMeta::new(new_account.pubkey(), false),
                AccountMeta::new_readonly(solana_system_interface::program::ID, false),
            ],
            data,
        }],
        Some(&bencher.payer().pubkey()),
        &[bencher.payer()],
        bencher.hash(),
    );
    bencher.execute_tx(IX_NAMES[3], tx);

    let data = if name == "star_frame" {
        discriminator("unchecked_accounts")
    } else {
        vec![4]
    };
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
                AccountMeta::new_readonly(Pubkey::new_unique(), false),
            ],
            data,
        }],
        Some(&bencher.payer().pubkey()),
        &[bencher.payer()],
        bencher.hash(),
    );
    bencher.execute_tx(IX_NAMES[4], tx);

    bencher.into_metrics()
}

pub fn main() {
    let pinocchio = runner("pinocchio");
    let anchor = runner("anchor");
    let typhoon = runner("typhoon");
    let star_frame = runner("star_frame");
    let result = generate_markdown([pinocchio, anchor, typhoon, star_frame]);

    let so_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../BENCHMARK.md"));

    println!("{}", so_path.to_str().unwrap());
    std::fs::write(so_path, result).expect("Failed to write benchmark results");
}

pub fn generate_markdown([pinocchio, anchor, typhoon, star_frame]: [BenchResult; 4]) -> String {
    let mut output = String::new();

    let format_cell = |val: u64, min: u64| -> String {
        if val == min {
            format!("🟩 **{val}**")
        } else if val <= min + (min / 2) {
            format!("🟩 {} (+{})", val, val - min)
        } else if val <= min * 2 {
            format!("🟨 {} (+{})", val, val - min)
        } else {
            format!("🟥 {} (+{})", val, val - min)
        }
    };

    output.push_str("## Benchmark Results\n\n");
    output.push_str("### Color Legend\n\n");
    output.push_str("- 🟩 **Green**: Best performance (minimum value) or within 50% of the best\n");
    output.push_str("- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)\n");
    output.push_str("- 🟥 **Red**: Poor performance (more than 2x the minimum value)\n\n");
    output.push_str("### CU Consumed\n\n");
    output.push_str(
        "| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |\n",
    );
    output.push_str(
        "| ------------- | --------------- | ----------------- | ------------ | -------------- |\n",
    );

    for key in IX_NAMES {
        let p_val = pinocchio.metrics.get(*key).unwrap_or(&0);
        let a_val = anchor.metrics.get(*key).unwrap_or(&0);
        let t_val = typhoon.metrics.get(*key).unwrap_or(&0);
        let s_val = star_frame.metrics.get(*key).unwrap_or(&0);

        let min_val = *p_val.min(a_val.min(t_val.min(s_val)));
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            key,
            format_cell(*p_val, min_val),
            format_cell(*a_val, min_val),
            format_cell(*t_val, min_val),
            format_cell(*s_val, min_val)
        ));
    }

    output.push_str("\n### Binary Size\n\n");
    output.push_str("|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |\n");
    output.push_str("| ------------------- | --------------- | ------------------- | -------- | -------------- |\n");

    let p_size = pinocchio.binary_size as u64;
    let a_size = anchor.binary_size as u64;
    let t_size = typhoon.binary_size as u64;
    let s_size = star_frame.binary_size as u64;
    let min_size = p_size.min(a_size.min(t_size.min(s_size)));

    output.push_str(&format!(
        "| Binary size (bytes) | {} | {} | {} | {} |\n",
        format_cell(p_size, min_size),
        format_cell(a_size, min_size),
        format_cell(t_size, min_size),
        format_cell(s_size, min_size)
    ));

    output
}
