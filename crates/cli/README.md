# Typhoon CLI

The Typhoon CLI is a command-line tool for scaffolding and managing Typhoon Solana projects.

## Commands

### `typhoon new`

Create a new Typhoon project workspace.

```bash
typhoon new <name> [--program <program-name>] [--path <path>] [--force] [--typhoon-path <path>]
```

**Options:**
- `name` - Project name (required)
- `--program` - Initial program name (optional)
- `--path` - Project directory path (defaults to current directory)
- `--force` - Force overwrite existing files
- `--typhoon-path` - Typhoon workspace path to use instead of the crate version

**Example:**
```bash
typhoon new my-project --program counter
```

### `typhoon add program`

Add a new program to an existing Typhoon workspace.

```bash
typhoon add program <name> [--path <path>]
```

**Options:**
- `name` - Program name (required)
- `--path` - Project directory path (defaults to current directory)

**Example:**
```bash
typhoon add program token --path ./my-project
```

### `typhoon add handler`

Add a new handler (instruction) to an existing program.

```bash
typhoon add handler --program <program-name> <name> [--path <path>]
```

**Options:**
- `--program` - Program name (required)
- `name` - Handler name (required)
- `--path` - Project directory path (defaults to current directory)

**Example:**
```bash
typhoon add handler --program counter increment --path ./my-project
```

## Usage

1. Create a new project:
   ```bash
   typhoon new my-solana-app
   cd my-solana-app
   ```

2. Add additional programs:
   ```bash
   typhoon add program token
   ```

3. Add handlers to programs:
   ```bash
   typhoon add handler --program counter increment
   ```

