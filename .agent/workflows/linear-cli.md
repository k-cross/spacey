---
description: Linear Ticket Context
---

# Read Issue and Parent Context

This workflow retrieves a specific Linear issue and automatically detects and fetches its parent issue (or project) to provide a complete context group.

## Trigger
- User asks to "read issue [ID] and its parent".
- User asks for "full context on issue [ID]".
- User asks to "get the issue group for [ID]".

## Steps

1. **Fetch Primary Issue**
   - Execute the CLI command to get details for the requested issue ID:
     ```bash
     linear-cli issues get <ISSUE_ID>
     ```
   - *Note: If `get` is invalid, try `view` or `show`. Use `linear-cli --help` if unsure.*

2. **Analyze Output for Parent**
   - **Agent Action:** Read the output from Step 1.
   - Look for fields labeled `Parent`, `Parent Issue`, or `Project`.
   - Extract the ID associated with that field (e.g., `ENG-101` or `Project: Rewrite Backend`).

3. **Fetch Parent Context (Conditional)**
   - **If a Parent Issue ID is found:**
     Run the command to fetch that specific issue:
     ```bash
     linear-cli issues get <PARENT_ISSUE_ID>
     ```
   - **If a Project is found (and relevant):**
     Run the command to fetch project details:
     ```bash
     linear-cli projects get "<PROJECT_NAME_OR_ID>"
     ```

4. **Summarize Context**
   - Confirm to the user that both the primary issue and its parent/project have been added to the context.
   - Example: "I've added context for issue ENG-123 and its parent, ENG-101."