```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Tsv2md {
        input: String,
    },

    Fastq {
        #[command(subcommand)]
        command: FastqCommand,
    },

    Vcf {
        #[command(subcommand)]
        command: VcfCommand,
    },
}

#[derive(Subcommand)]
enum FastqCommand {
    Stats {
        input: String,
    },

    Filter {
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Tsv2md { input } => {
            biox_tsv::to_markdown(&input);
        }

        Command::Fastq { command } => {
            // ...
        }

        Command::Vcf { command } => {
            // ...
        }
    }
}
```

```
biox/
├── crates/
│   ├── biox-core/
│   ├── biox-fastq/
│   └── biox-vcf/
│
└── bins/
    ├── biox/
    ├── biox-server/
    └── biox-worker/
```

```
biox/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
│
├── crates/
│   ├── biox-core/
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── biox-tsv/
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── biox-fastq/
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── biox-vcf/
│   │   └── src/
│   │       └── lib.rs
│   │
│   └── biox-fasta/
│       └── src/
│           └── lib.rs
│
└── src/
    ├── main.rs
    └── commands/
        ├── mod.rs
        ├── tsv.rs
        ├── fastq.rs
        ├── vcf.rs
        └── fasta.rs
```