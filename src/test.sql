SELECT
  "directus_permissions"."id",
  CAST("directus_permissions"."role" AS text),
  "directus_permissions"."collection",
  "directus_permissions"."action",
  "directus_permissions"."permissions",
  "directus_permissions"."validation",
  "directus_permissions"."presets",
  "directus_permissions"."fields"
FROM
  "directus_permissions"
WHERE
  "directus_permissions"."collection" LIKE 'direc_%';

SELECT
  "directus_permissions"."id",
  CAST("directus_permissions"."role" AS text),
  "directus_permissions"."collection",
  "directus_permissions"."action",
  "directus_permissions"."permissions",
  "directus_permissions"."validation",
  "directus_permissions"."presets",
  "directus_permissions"."fields"
FROM
  "directus_permissions"
WHERE
  "directus_permissions"."collection" LIKE '%'
  AND (
    (
      "directus_permissions"."fields" NOT LIKE ','
      AND "directus_permissions"."fields" LIKE 'test'
    )
    OR "directus_permissions"."fields" LIKE '%,test,%'
    OR "directus_permissions"."fields" LIKE 'test%'
    OR "directus_permissions"."fields" LIKE '%test'
    OR "directus_permissions"."fields" = '*'
  );

SELECT
  "directus_permissions"."id",
  CAST("directus_permissions"."role" AS text),
  "directus_permissions"."collection",
  "directus_permissions"."action",
  "directus_permissions"."permissions",
  "directus_permissions"."validation",
  "directus_permissions"."presets",
  "directus_permissions"."fields"
FROM
  "directus_permissions"
WHERE
  "directus_permissions"."collection" = 'thing'
  AND (
    (
      "directus_permissions"."fields" NOT LIKE ','
      AND "directus_permissions"."fields" LIKE 'c%nten%'
    )
    OR "directus_permissions"."fields" LIKE '%,c%nten%,%'
    OR "directus_permissions"."fields" LIKE 'c%nten%%'
    OR "directus_permissions"."fields" LIKE '%c%nten%'
    OR "directus_permissions"."fields" = '*'
  )