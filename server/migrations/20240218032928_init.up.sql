-- Add up migration script here
CREATE TABLE "users" (
    user_id UUID NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password TEXT NOT NULL,
    preferred_platform TEXT,
    photo TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE "songs" (
    song_id UUID NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    artist_id UUID NOT NULL,
    album_id UUID NOT NULL,
    duration SMALLINT NOT NULL,
    genre TEXT NOT NULL,
    external_url TEXT[] NOT NULL
);

CREATE TABLE "artists" (
    artist_id UUID NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    genre TEXT NOT NULL
);

CREATE TABLE "albums" (
    album_id UUID NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    artist_id UUID NOT NULL,
    release_date TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE "user_preferences" (
    preference_id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL,
    song_id UUID NOT NULL,
    artist_id UUID NOT NULL,
    album_id UUID NOT NULL
);

CREATE TABLE "recommendations" (
    recommendation_id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL,
    song_id UUID NOT NULL,
    match_score REAL NOT NULL
);