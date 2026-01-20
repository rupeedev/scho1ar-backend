# Update Getting Started Tutorial

Update the backend tutorial documentation to reflect current project state and recent changes.

## Arguments

- `$ARGUMENTS` - (Optional) Specific section or topic to update (e.g., "docker", "endpoints", "environment")

## Tutorial Location

```
/Users/rupeshpanwar/Documents/docs/docs-scho1ar/backend-tutorial/01-getting-started.md
```

## Instructions

### 1. Read Current State

First, gather information about the current project state:

```bash
# Check project structure
ls -la /Users/rupeshpanwar/Documents/Projects/Schol1ar.com/scho1ar-backend/

# Check Docker files
cat Dockerfile docker-compose.yml 2>/dev/null || echo "No Docker files"

# Check current endpoints
cat src/routes/mod.rs

# Check environment variables
cat .env.example

# Check Cargo.toml for dependencies
cat Cargo.toml
```

### 2. Read Current Tutorial

```
/Users/rupeshpanwar/Documents/docs/docs-scho1ar/backend-tutorial/01-getting-started.md
```

### 3. Update Sections

Based on `$ARGUMENTS` or full update if not specified:

#### TL;DR Section
- Ensure all quick commands are current and working
- Add any new essential commands
- Remove deprecated commands

#### Current Running Status
- Update component status table
- Verify ports and container names

#### Project Structure
- Add any new files to the tree
- Update file descriptions if purposes changed
- Remove deleted files

#### Tech Stack
- Add new technologies/dependencies
- Update versions if changed

#### Docker Setup (if applicable)
- Update Docker summary table
- Add new Docker commands if needed
- Update Dockerfile/compose descriptions
- Verify image sizes

#### Native Setup
- Update prerequisites (Rust version, etc.)
- Verify all steps still work

#### Available Endpoints
- Add new endpoints
- Remove deprecated endpoints
- Update response examples if changed

#### Environment Variables
- Add new variables
- Update defaults
- Remove deprecated variables

#### Development Commands
- Add new useful commands
- Update flags/options if changed

#### Troubleshooting
- Add new common issues encountered
- Update solutions if better approaches found

#### Next Steps
- Update priority list based on progress
- Mark completed items
- Add new planned items

### 4. Verify Changes

After updating, verify the documentation is accurate:

```bash
# Test health endpoint
curl -s http://localhost:3001/health | jq .

# Test Docker if running
docker-compose ps 2>/dev/null

# Check server status
ps aux | grep scho1ar-backend
```

### 5. Update Metadata

Update the "Last Updated" date at the top of the document to today's date.

## Output Format

After completing updates, provide a summary:

```
Tutorial Updated: 01-getting-started.md

Sections Modified:
- [Section name]: [What changed]
- [Section name]: [What changed]

Sections Unchanged:
- [Section name]: No changes needed

Verified:
- [ ] TL;DR commands tested
- [ ] Endpoints verified
- [ ] Docker setup confirmed
```

## Example Usage

```
/update-tutorial                    # Full update
/update-tutorial docker             # Update Docker section only
/update-tutorial endpoints          # Update endpoints section only
/update-tutorial troubleshooting    # Add new troubleshooting items
```
