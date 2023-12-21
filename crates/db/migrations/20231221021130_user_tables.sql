-- migrate:up

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    hashed_password VARCHAR NOT NULL,
    reset_password_selector VARCHAR,
    reset_password_sent_at TIMESTAMP,
    reset_password_validator_hash VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO users(email, hashed_password) VALUES('test1@test1.com', 'aasdsaddasad');
INSERT INTO users(email, hashed_password) VALUES('test2@test1.com', 'aasdsaddasad');
INSERT INTO users(email, hashed_password) VALUES('test3@test1.com', 'aasdsaddasad');

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    session_verifier VARCHAR NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otp_code_encrypted VARCHAR NOT NULL,
    otp_code_attempts INTEGER NOT NULL DEFAULT 0,
    otp_code_confirmed BOOLEAN NOT NULL DEFAULT false,
    otp_code_sent BOOLEAN NOT NULL DEFAULT false
);

COMMENT ON TABLE sessions IS 'The users login sessions';
COMMENT ON COLUMN sessions.session_verifier IS ' The session is a 32 byte random number stored in their cookie. This is the sha256 hash of that value.';
COMMENT ON COLUMN sessions.otp_code_encrypted IS 'A 6 digit code that is encrypted here to prevent attackers with read access to the database being able to use it.';
COMMENT ON COLUMN sessions.otp_code_attempts IS 'We count OTP attempts to prevent brute forcing.';
COMMENT ON COLUMN sessions.otp_code_confirmed IS 'Once the user enters the correct value this gets set to true.';
COMMENT ON COLUMN sessions.otp_code_sent IS 'Have we sent the OTP code?';


-- migrate:down
DROP TABLE users;
DROP TABLE sessions;