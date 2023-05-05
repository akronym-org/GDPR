# GDPR – Granular Directus Permissions Resolver

A CLI that helps you wrangle hundreds of Directus permissions.

Directus Permissions are administered and displayed per role. But what if you need verify WHO
has access to a specific column? What if you have hundreds of roles and need to change access
to a new column in a specific way?

The goal is to have a tool that can be used in dev envs and CI/CD.

## Limitations

* GDPR is pre-alpha. Even `gdpr dump` doesn't work properly yet.
* This tool doesn't have a security audit. Don't use it!
* GDPR uses SeaORM, which supports Postgres, MySQL and SQLite.

## Reading data

Connect to DB `gdpr dump -u postgres://user:pwd@localhost:5432/mydb` and display permissions as JSON.

Dump to yaml `gdpr dump -u postgres://user:pwd@localhost:5432/mydb -o yaml`.

You can save permissions by redirecting to a file.
`gdpr dump -u postgres://user:pwd@localhost:5432/mydb -o yaml > permissions.yaml`

Use `--table table_name` or `--t` for selecting one specific table with all its columns.

Use `--field table.column` or `--f` for selecting one specific column.

Combining both `--table` and `--field` means, you can't use dot notation in the field option.

`gdpr dump -t table_name -f field_name`

This will output the permissions for table_name.field_name and is the same as:

`gdpr dump -f table_name.field_name`

### Output format

GDPR will deduplicate same permissions and validations for multiple roles.

The format used by GDPR looks like this (for table_name.field_name)

```json
{
  "field_name": {
    "create": [
      {
        "roles": [ "role_name" ],
        "_and": [
          {
            "field_name": {
              "_ncontains": "dirtyword"
            }
          },
          {
            "id": { "_lt": "50" }
          }
        ]
      },
    ],
    "read": [ 
      {
        "roles": [ "role_name" ],
        "_and": [{
          "id": { "_lt": "50" }
        }]
      }
    ],
    "update": [
      {
        "roles": [ "role_name" ],
        "permissions": {
          "_and": [{
            "id": { "_lt":"50" }
          }]
        },
        "validation": {
          "_and": [{
            "field_name": {
              "_ncontains": "dirtyword"
            }
          }]
        }
      },
    ],
    "delete": [],
    "share": []
  }
}
```

Or as yaml

```yaml
field_name:
  create:
  - roles: role_name
    permissions:
    - _and:
      - field_name:
        _ncontains: dirtyword
      - id:
        - lt: 50
  read:
  - roles: role_name
    permissions:
    - _and:
      - id:
        - lt: 50
  update:
  - roles: role_name
    permissions:
    - _and:
      - id:
        - lt: 50
    validation:
    - _and:
      - field_name:
        _ncontains: dirtyword
  delete:
  share:
```

If you use option `--simple` (TODO: find better option name) you can simply display the CRUD actions as:

```yaml
field_name:
  create: role_name
  read: role_name
  update: role_name
  delete:
  share:
```

## Updating permissions

You can also update permissions.
In order to replace all permissions with an updated permissions set use

```bash
gdpr replace -u postgres://user:pwd@localhost:5432/mydb < permissions.yml
```

Commit the permissions yaml to git or test if the permissions are reflected by reality in CI/CD.

If you want to update only specific columns, because you're quickly iterating and in a dev env:

```bash
gdpr patch -u postgres://user:pwd@localhost:5432/mydb --field table '{ "*_role": { "read": "ALL", "create": "ALL", "update": "ALL", "delete": "ALL" } }'
```

or

```bash
gdpr patch -u postgres://user:pwd@localhost:5432/mydb --field table.column '{ "*_role": "ALL" }'
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
