-- Add up migration script here
DO $$ BEGIN
 CREATE TYPE "platform" AS ENUM('APPLE_MUSIC', 'SPOTIFY', 'YOUTUBE_MUSIC', 'AMAZON_MUSIC', 'SOUNDCLOUD', 'TIDAL');
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 CREATE TYPE "genre" AS ENUM('Pop', 'Rock', 'Hip-Hop', 'Rap', 'Jazz', 'Classical', 'Country', 'Electronic', 'Dance', 'R&B', 'Soul', 'Reggae', 'Folk', 'Blues', 'Latin', 'Metal', 'Punk', 'Indie', 'Alternative', 'World', 'K-Pop', 'Anime', 'Children', 'Holiday');
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint

