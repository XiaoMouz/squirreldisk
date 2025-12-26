# File Ignore Feature

## Overview
This feature allows users to exclude specific files and folders from disk scans by defining ignore patterns.

## UI Preview

```
┌────────────────────────────────────────────────────────────┐
│  SquirrelDisk                           ⚙️ Settings   ×    │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  When settings button is clicked:                         │
│                                                            │
│  ┌─────────── Settings Modal ───────────────────┐        │
│  │                                                │        │
│  │  Ignore Patterns                          ×   │        │
│  │  ────────────────────────────────────────────│        │
│  │                                                │        │
│  │  Add patterns to ignore files and folders     │        │
│  │  during scans. Use wildcards like *.log,      │        │
│  │  node_modules, etc.                           │        │
│  │                                                │        │
│  │  ┌──────────────────────────┐  [Add]         │        │
│  │  │ e.g., *.log, .git...     │                │        │
│  │  └──────────────────────────┘                │        │
│  │                                                │        │
│  │  Configured Patterns:                         │        │
│  │  ┌────────────────────────────────────────┐  │        │
│  │  │ ☑ node_modules            [Remove]    │  │        │
│  │  │ ☑ *.log                   [Remove]    │  │        │
│  │  │ ☐ .git                    [Remove]    │  │        │
│  │  └────────────────────────────────────────┘  │        │
│  │                                                │        │
│  │  [Close]                                      │        │
│  └────────────────────────────────────────────────┘        │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## How to Use

### Accessing Settings
1. Click the settings icon (gear icon) in the title bar
2. The Settings dialog will open

### Adding Ignore Patterns
1. Enter a pattern in the input field (e.g., `*.log`, `node_modules`, `.git`)
2. Press Enter or click the "Add" button
3. The pattern will be added to the list and saved automatically

### Managing Patterns
- **Toggle**: Check/uncheck the checkbox to enable/disable a pattern
- **Remove**: Click the "Remove" button to delete a pattern permanently

### Pattern Syntax
Patterns support simple wildcard matching:
- `*` - Matches any characters (e.g., `*.log` matches all .log files)
- `?` - Matches a single character
- Exact names - Match folder/file names exactly (e.g., `node_modules`, `.git`)

### Examples
- `*.log` - Ignores all files ending with .log
- `*.tmp` - Ignores all temporary files
- `node_modules` - Ignores node_modules directories
- `.git` - Ignores .git directories
- `build` - Ignores build directories
- `.cache` - Ignores .cache directories

## Technical Details

### Backend (Rust)
- **Config Storage**: Patterns are saved in `{app_config_dir}/ignore_patterns.json`
- **Filtering**: Applied post-scan to the JSON tree structure
- **Commands**: 
  - `get_ignore_patterns` - Retrieves all patterns
  - `add_ignore_pattern` - Adds a new pattern
  - `remove_ignore_pattern` - Removes a pattern
  - `toggle_ignore_pattern` - Enables/disables a pattern

### Frontend (React)
- **Settings Component**: Modal dialog for managing patterns
- **TitleBar Integration**: Settings button in the title bar
- **Real-time Updates**: Changes are saved immediately

### Pattern Matching
- Patterns are matched against file and folder names
- Case-sensitive matching
- Supports both file names and directory names
- Filters are applied recursively through the file tree
