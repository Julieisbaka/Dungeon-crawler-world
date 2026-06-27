---
name: content-schema-auditor
description: >-
  Skill for validating and editing game content JSON and schema files to keep data
  structure, references, and documentation consistent.
user-invocable: true
---

# Content and Schema Auditor Skill

Use this skill for tasks touching JSON content and schema definitions.

## Scope

- Content folders such as `Items/`, `Skills/`, `Classes_and_Races/`, `Floor/`, `Magic/`, and related directories.
- Schema files under `Scheme/`.

## Expectations

- Keep JSON/schema changes backwards-compatible when possible.
- Ensure required fields, enums, and references are consistent.
- Keep linked descriptions and documentation paths valid.

## Validation

- Run `cargo build` and `cargo test` if runtime data is changed.
- Verify modified JSON files still align with the relevant schema structure.
