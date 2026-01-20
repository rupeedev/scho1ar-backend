# Update Project Documentation

Update all `.claude/` documentation files to reflect current project state.

## Instructions

Perform the following updates in sequence:

### 1. Update File Map (`.claude/filemap.md`)

Scan the project structure and update the filemap:

```bash
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.sql" \) | grep -v "/target/" | sort
```

- Add any new source files with descriptions
- Add any new directories to the structure
- Update "Quick Reference" line numbers if code has moved
- Update "Planned Directories" section (move implemented items to main structure)

### 2. Update Tech Stack (`.claude/techstack.md`)

Check for dependency changes:

```bash
cat Cargo.toml
```

- Add any new dependencies to the appropriate table
- Move items from "Planned Additions" to main tables if implemented
- Update version numbers if upgraded

### 3. Update Coding Guidelines (`.claude/coding-guidelines.md`)

Review recent code changes for new patterns:

- Add new patterns that have been established
- Document any new conventions adopted
- Add new "Common Pitfalls" if discovered

### 4. Update Lessons Learned (`.claude/lessons-learned.md`)

If there were any significant bugs or issues resolved in this session:

- Add new incident entries with:
  - Date
  - Incident description
  - Root cause
  - Solution
  - Prevention steps

### 5. Update CLAUDE.md

If any major architectural changes occurred:

- Update the "Application Flow" diagram
- Update "Key Components" table
- Update "Environment Variables" if new ones added

## Output

After completing updates, provide a summary:

```
Documentation Updated:
- filemap.md: [what changed]
- techstack.md: [what changed]
- coding-guidelines.md: [what changed]
- lessons-learned.md: [what changed]
- CLAUDE.md: [what changed]
```

If no changes needed for a file, note "No changes required".
