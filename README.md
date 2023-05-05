# GDPR – Granular Directus Permissions Resolver

A CLI that helps you wrangle & audit hundreds of Directus permissions.

Directus Permissions are administered and displayed per role. But what if you need verify WHO
has access to a specific column? What if you have hundreds of roles and need to change access
to a new column in a specific way?

GDPR's goals are:

* allows you to quickly iterate in dev
* can be used for tests CI/CD
* makes auditing Directus deployments easier

## Limitations

* Currently only works with Postgres.
* GDPR is pre-alpha. Only `dump` works.
* You must specify a table and a field.
* This tool doesn't have a security audit. Don't use it!
* GDPR uses SeaORM, which supports Postgres, MySQL and SQLite.

## Auditing permissions

Connect to your database

```bash
gdpr dump -u postgres://user:pwd@localhost:5432/mydb
```

and display all permissions as JSON.

Dump your permissions to yaml with `-o yaml`. You can save your output by redirecting to a file.

```bash
gdpr dump -o yaml > permissions.yaml
```

### Inspect specific tables & columns

Audit one specific column like this:

```bash
`gdpr dump -t table_name -f field_name`
```

Use only `--table table_name` or `--t` for selecting all columns of a table. (TODO: support wildcards like `directus_*`)

You can also use:

```bash
gdpr dump -f table_name.field_name
```

You cannot specify `--table some_table` and use the dot-notation in `--field`.

### Output format

GDPR will deduplicate the same permissions and validations for multiple roles.

The format used by GDPR looks like this (for table_name.field_name)

```json
{
  "field_name": {
    "create": [
      {
        "roles": [ "public" ],
        "permissions": {},
        "validation": {},
      },
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
  - roles:
    - role_name
    - role_name_2
    permissions:
    - _and:
      - field_name:
        _ncontains: dirtyword
      - id:
        - lt: 50
  read:
  - roles:
    - role_name
    permissions:
    - _and:
      - id:
        - lt: 50
  update:
  - roles:
    - role_name
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
  create: role_name, role_name_2
  read: role_name
  update: role_name
  delete:
  share:
```

## ~~Updating permissions~~

You can also (**not yet**) update permissions.
In order to replace all permissions with an updated permissions set use

```bash
gdpr replace < permissions.yml
```

Commit the permissions yaml to git or test if the permissions are reflected by reality in CI/CD.

If you want to update only specific columns, because you're quickly iterating and in a dev env:

```bash
gdpr patch -f table.column --role '*_role' '{ "read": "ALL", "create": "ALL", "update": "ALL" }'
```

If you don't mind also adding permissions for `delete` and `share` you could also simply run:

```bash
gdpr patch -f table.column --role '*_role' ALL
```

You can match roles with regex. In the above example `"*_role"` matches all roles that have the
suffix `_role`

A short version for granting all access to all roles is `{ "*": { "*": "ALL" } }`

You can also update from a file

```bash
gdpr patch -f table.column < patch.yml
```

### Env vars

If you want to avoid specifying connection details `--url postgres://user:pwd@localhost:5432/mydb`
every time, you can:

* Set env vars
* Use an .env file

Additionally, you can call GDPR with a prepended env var. Currently it's `DATABASE_URL`.

```bash
DATABASE_URL=postgres://user:pwd@localhost:5432/mydb gdpr dump
```
