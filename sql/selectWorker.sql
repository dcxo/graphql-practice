SELECT
    id as "id!: Uuid",
    name,
    last_name,
    created_time as "created_time!: DateTime<Utc>",
    role as "role!: Role",
    department_id as "department_id!: Uuid"
FROM
    Workers
WHERE
    name LIKE '%' || ? || '%'