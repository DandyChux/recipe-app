-- Add down migration script here
DO $$
BEGIN
    DROP TABLE IF EXISTS platforms;
EXCEPTION
    WHEN others THEN
        RAISE NOTICE 'Table "platforms" does not exist.';
END
$$;