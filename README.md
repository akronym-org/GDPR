# GDPR – Granular Directus Permissions Resolver

**Warning: GDPR is pre-alpha**

A CLI that helps you wrangle & audit hundreds of 🐇 [Directus](https://directus.io) permissions.

📚 Read the Docs at: [gdpr.akronym.io](https://gdpr.akronym.io)

## The problem

Directus Permissions are administered and displayed per role.

What if you want to verify WHO has access to a specific column?

What if you have 20 roles and want to add specific access to a new column?

## Goals

* Quickly iterate in your dev env
* Verify permissions during/after deployment in your pipeline
* Audit your Directus deployment

## Limitations

* Currently only works with Postgres
* GDPR is pre-alpha. Only `dump` works
* You can & must currently specify one `--table`
* This tool doesn't have a security audit. Don't use it!
* GDPR uses SeaORM, which supports Postgres, MySQL and SQLite.

## Roadmap

The available features are (going to be):

* show/save permissions per table, per field, with wildcards
* `replace` / `patch`: update permissions, granular/universal, with wildcards
* easily seed roles and permissions
