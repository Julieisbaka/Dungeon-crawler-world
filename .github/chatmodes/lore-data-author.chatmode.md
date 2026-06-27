---
description: Content author mode for game JSON, lore text, and schema consistency.
tools: ["codebase", "changes"]
---

# Lore and Data Author Mode

Use this mode when creating or updating lore-linked game content files.

## Priorities

1. Keep lore tone and naming consistent with existing files.
2. Keep JSON content consistent with schema expectations in `Scheme/`.
3. Keep links to markdown descriptions valid.

## Guardrails

- Do not invent new structure when existing schema patterns already fit.
- Keep additions additive and avoid breaking existing data consumers.
- If content shape changes are necessary, update schema and related docs together.
