# Add Lesson Learned

Document a new lesson learned from a bug, issue, or discovery.

## Arguments

- `$ARGUMENTS` - Brief description of the incident

## Instructions

1. Read the current lessons-learned file:
   ```
   .claude/lessons-learned.md
   ```

2. Add a new entry at the end with today's date in this format:

```markdown
---

## YYYY-MM-DD: [Title of Incident]

### Incident

[What happened - error message, unexpected behavior, etc.]

### Root Cause

[Why it happened - the underlying issue]

### Solution

[How it was fixed - code example if applicable]

### Prevention

- [How to avoid this in the future]
- [What to check before similar changes]
```

3. If the lesson relates to a common pattern, also consider updating:
   - `.claude/coding-guidelines.md` - Add to "Common Pitfalls" section
   - `CLAUDE.md` - Add warning if it's critical

## Example

Input: `CORS wildcard with credentials`

Output added to lessons-learned.md:
```markdown
## 2026-01-20: CORS Wildcard with Credentials

### Incident
Server crashed with panic about invalid CORS configuration...
```
