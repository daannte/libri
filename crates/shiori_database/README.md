# shiori_database

This crate contains the shiori database schema.

## `schema.patch`

### 1. Temporarily disable `patch_file` in `diesel.toml`

Comment out the `patch_file` line to prevent Diesel from automatically applying it during schema generation:

```toml
# patch_file = "..."
```

### 2. Generate the current schema

Run Diesel’s `print-schema` command to refresh `src/schema.rs`:

```bash
diesel print-schema > src/schema.rs
```

### 3. Backup the generated schema

Make a copy of the original generated schema for comparison later:

```bash
cp src/schema.rs src/schema.rs.orig
```

### 4. Apply the patch

Apply the patch file:

```bash
patch src/schema.rs src/schema.patch
```

### 5. Update the patch file

After making manual fixes to `schema.rs`, regenerate the patch file for future use:

```bash
diff -Naur --label original --label patched src/schema.rs.orig src/schema.rs > src/schema.patch
```

### 6. Re-enable the patch in `diesel.toml`

Once the patch is updated, uncomment the line in `diesel.toml`:

```toml
patch_file = "..."
```
