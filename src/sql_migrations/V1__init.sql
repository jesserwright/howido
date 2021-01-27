CREATE TABLE "howto" (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title varchar(80) NOT NULL
);

CREATE TABLE "step" (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title varchar(100) NOT NULL,
    image_filename varchar(255) NOT NULL
);

-- what is the primary key of this table?
CREATE TABLE "howto_step" (
    -- TODO: add a list position.
    position int NOT NULL,
    step_id int REFERENCES "step",
    howto_id int REFERENCES "howto"
);

-- TODO: update the db with these
-- TODO: later - use refinery for migrations. Schema should be fairly well defined though.