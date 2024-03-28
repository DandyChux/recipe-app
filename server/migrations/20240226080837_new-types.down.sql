-- Add down migration script here
DO $$ BEGIN
    DROP TYPE IF EXISTS "platform";
EXCEPTION
    WHEN undefined_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
    DROP TYPE IF EXISTS "genre";
EXCEPTION
    WHEN undefined_object THEN null;
END $$;
--> statement-breakpoint-- Add down migration script here
