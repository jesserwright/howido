CREATE TABLE "instruction" (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title varchar(80) NOT NULL
);

CREATE TABLE "step" (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title varchar(100) NOT NULL,
    seconds int NOT NULL
);

-- what is the primary key of this table?
CREATE TABLE "instruction_step" (
    step_id int REFERENCES "step",
    instruction_id int REFERENCES "instruction"
);
