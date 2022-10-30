-- CREATE EXTENSION IF NOT EXISTS dblink;

-- DO $$
-- BEGIN
-- PERFORM dblink_exec('', 'CREATE DATABASE hotel_service');
-- EXCEPTION WHEN duplicate_database THEN RAISE NOTICE '%, skipping', SQLERRM USING ERRCODE = SQLSTATE;
-- END
-- $$;


-- use hotel_service;

CREATE SCHEMA IF NOT EXISTS hotel_service;

CREATE TABLE IF NOT EXISTS hotel_service.hotel(
    hotel_id serial PRIMARY KEY,
    name VARCHAR,
    address VARCHAR,
    location VARCHAR
);

CREATE TABLE IF NOT EXISTS hotel_service.room(
    hotel_id serial PRIMARY KEY,
    room_type_id int,
    name VARCHAR,
    address VARCHAR,
    location VARCHAR
);