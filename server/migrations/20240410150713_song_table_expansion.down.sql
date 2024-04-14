-- Add down migration script here
ALTER TABLE "songs" DROP COLUMN "tempo";
ALTER TABLE "songs" DROP COLUMN "time_signature";
ALTER TABLE "songs" DROP COLUMN "key";
ALTER TABLE "songs" DROP COLUMN "loudness";
ALTER TABLE "songs" DROP COLUMN "speechiness";
ALTER TABLE "songs" DROP COLUMN "danceability";
ALTER TABLE "songs" ALTER COLUMN "genre" SET DATA TYPE TEXT;

ALTER TABLE "artists" DROP COLUMN "genres";
ALTER TABLE "artists" DROP COLUMN "albums";
ALTER TABLE "artists" DROP COLUMN "tracks";
ALTER TABLE "artists" ADD COLUMN "genre" TEXT;

ALTER TABLE "albums" DROP COLUMN "genre";
ALTER TABLE "albums" DROP COLUMN "tracks";
ALTER TABLE "albums" DROP COLUMN "cover";