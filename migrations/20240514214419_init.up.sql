CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "users" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        name VARCHAR(100) NOT NULL,
        subscribed BOOLEAN NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(100) NOT NULL,
        admin BOOLEAN NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );


CREATE TABLE
    "posts" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        -- premium BOOLEAN NOT NULL,
        category VARCHAR(255), 
        title VARCHAR(100) NOT NULL UNIQUE,
        subtitle VARCHAR(255),
        content TEXT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );

CREATE INDEX users_email_idx ON users (email);
