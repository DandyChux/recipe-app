-- Add up migration script here
DO $$
BEGIN
    CREATE TABLE platforms (
        platform_id UUID PRIMARY KEY,
        name PLATFORM NOT NULL,
        url TEXT NOT NULL
    );
EXCEPTION
    WHEN duplicate_table THEN
        RAISE NOTICE 'Table "platforms" already exists.';
END
$$;