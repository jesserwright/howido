BEGIN;

CREATE TABLE "howto" (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title varchar(80) NOT NULL
);

CREATE TABLE "step" (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title varchar(100) NOT NULL,
    image_filename varchar(255) NOT NULL
);

CREATE TABLE "howto_step" (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    position int NOT NULL,
    step_id int REFERENCES "step",
    howto_id int REFERENCES "howto"
);

-- Seed a howto to make inserting steps possible. Seeding should eventually be done by an api, like an integration test.
-- So, maybe howto creation should be the next thing I work on. Something different.
INSERT INTO howto (title) VALUES ('first');

COMMIT;
