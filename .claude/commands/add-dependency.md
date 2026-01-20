# Add Dependency

Add a new crate dependency and document it.

## Arguments

- `$ARGUMENTS` - Crate name and purpose (e.g., "jsonwebtoken for JWT auth")

## Instructions

### 1. Add to Cargo.toml

Parse the argument to extract crate name and add to `Cargo.toml`:

```bash
cargo add {crate_name}
```

Or manually add with specific features if needed.

### 2. Verify Installation

```bash
cargo check
```

### 3. Update Tech Stack Documentation

Add the new dependency to `.claude/techstack.md`:

- Add to appropriate table (Web Framework, Database, Utilities, etc.)
- Include purpose/description
- If it was in "Planned Additions", move it to main section

### 4. Update Coding Guidelines (if applicable)

If the crate introduces new patterns, add examples to `.claude/coding-guidelines.md`.

### 5. Summary

Output:
```
Added dependency: {crate_name}
- Updated Cargo.toml
- Updated .claude/techstack.md
- cargo check: [passed/failed]
```
