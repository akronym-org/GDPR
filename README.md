# GDPR – Granular Directus Permissions Resolver

Directus Permissions are administered and displayed per role. But what if you need to check WHO
has access to a specific column? What if you have hundreds of roles and need to change access
to a new column in a specific way?

## Limitations

This tool doesn't have a security audit. Do not use in production.

GDPR uses sqlx for connecting to databases. It supports Postgres, MySQL, SQLite and MSSQL.

## How to

### Reading data

Connect to DB `gdpr dump -u postgres://user:pwd@localhost:5432/mydb` and display permissions as JSON.

Dump yaml to output `gdpr dump -u postgres://user:pwd@localhost:5432/mydb -o yaml > output.yml`.

Use `--field table.column` or `--f` for outputting one specific column.

Use `--table tablename` or `--t` for outputting one specific table with all columns.

### Updating permissions

You can also update permissions.
In order to replace all permissions with an updated permissions set use

```bash
gdpr replace -u postgres://user:pwd@localhost:5432/mydb < permissions.yml
```

Commit the permissions yaml to git or test if the permissions are reflected by reality in CI/CD.

If you want to update only specific columns, because you're quickly iterating and in a dev env:

```bash
gdpr patch -u postgres://user:pwd@localhost:5432/mydb --field table.column '{ "*_role": { "read": "ALL", "create": "ALL", "update": "ALL", "delete": "ALL" } }'
```

You can match roles with regex. In the above example `"*_role"` matches all roles that have the
suffix `_role`

A short version for granting all access to all roles is `{ "*": { "*": "ALL" } }`

You can also update from a file

```bash
gdpr patch -u postgres://user:pwd@localhost:5432/mydb --field table.column < patch.yml
```

### Env vars

If you want to avoid specifying connection details `-h localhost -p 5432 -U postgres`, you can:

* Set env var
* Set an .env file
* Call `gdpr` with env var prepended
* You can use env var DATABASE_URL

```bash
DATABASE_URL=postgres://user:pwd@localhost:5432/mydb gdpr dump
```
