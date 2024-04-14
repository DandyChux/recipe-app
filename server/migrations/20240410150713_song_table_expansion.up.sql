-- Add up migration script here
ALTER TABLE "songs" ADD COLUMN "tempo" REAL; --> statement-breakpoint
ALTER TABLE "songs" ADD COLUMN "time_signature" SMALLINT; --> statement-breakpoint
ALTER TABLE "songs" ADD COLUMN "key" SMALLINT; --> statement-breakpoint
ALTER TABLE "songs" ADD COLUMN "loudness" REAL; --> statement-breakpoint
ALTER TABLE "songs" ADD COLUMN "speechiness" REAL; --> statement-breakpoint
ALTER TABLE "songs" ADD COLUMN "danceability" REAL; --> statement-breakpoint
ALTER TABLE "songs" ALTER COLUMN "genre" TYPE genre USING genre::genre; --> statement-breakpoint

ALTER TABLE "artists" DROP COLUMN IF EXISTS "genre"; --> statement-breakpoint
ALTER TABLE "artists" ADD COLUMN "genres" genre[]; --> statement-breakpoint
ALTER TABLE "artists" ADD COLUMN "albums" UUID[]; --> statement-breakpoint
ALTER TABLE "artists" ADD COLUMN "tracks" UUID[]; --> statement-breakpoint

ALTER TABLE "albums" ADD COLUMN "genre" genre; --> statement-breakpoint
ALTER TABLE "albums" ADD COLUMN "tracks" UUID[]; --> statement-breakpoint
ALTER TABLE "albums" ADD COLUMN "cover" TEXT; --> statement-breakpoint