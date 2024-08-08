-- Creating the 'member' table
CREATE TABLE member (
    member_id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,  -- Assuming hash storage for password
    full_name VARCHAR(255),
    phone_number VARCHAR(20),
    profile_pic VARCHAR(255)
);

-- Creating the 'admin' table
CREATE TABLE admin (
    admin_id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    phone_number VARCHAR(20),
    admin_level INTEGER CHECK (admin_level BETWEEN 1 AND 3),
    profile_pic VARCHAR(255)
);

CREATE TABLE posts (
    post_id SERIAL PRIMARY KEY,
    post_title VARCHAR(255) NOT NULL,
    post_content TEXT NOT NULL,
    post_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    like_count INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    author_type VARCHAR(100) NOT NULL,
    author_id INTEGER NOT NULL,
    CHECK (author_type IN ('member', 'admin')),
    FOREIGN KEY (author_id) REFERENCES member(member_id) ON DELETE SET NULL DEFERRABLE INITIALLY DEFERRED
);

-- Trigger function to enforce the author_type constraint
CREATE OR REPLACE FUNCTION enforce_author_type_posts()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.author_type = 'admin' THEN
        IF NOT EXISTS (SELECT 1 FROM admin WHERE admin_id = NEW.author_id) THEN
            RAISE EXCEPTION 'Invalid admin_id for author_type admin';
        END IF;
    ELSIF NEW.author_type = 'member' THEN
        IF NOT EXISTS (SELECT 1 FROM member WHERE member_id = NEW.author_id) THEN
            RAISE EXCEPTION 'Invalid member_id for author_type member';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to enforce the author_type constraint
CREATE TRIGGER enforce_author_type_trigger_posts
BEFORE INSERT OR UPDATE ON posts
FOR EACH ROW EXECUTE FUNCTION enforce_author_type_posts();

CREATE TABLE comments (
    comment_id SERIAL PRIMARY KEY,
    comment_content VARCHAR(255) NOT NULL,
    comment_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    author_type VARCHAR(100) NOT NULL,
    author_id INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    CHECK (author_type IN ('member', 'admin')),
    FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE
);

-- Trigger function to enforce the author_type constraint
CREATE OR REPLACE FUNCTION enforce_author_type_comments()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.author_type = 'admin' THEN
        IF NOT EXISTS (SELECT 1 FROM admin WHERE admin_id = NEW.author_id) THEN
            RAISE EXCEPTION 'Invalid admin_id for author_type admin';
        END IF;
    ELSIF NEW.author_type = 'member' THEN
        IF NOT EXISTS (SELECT 1 FROM member WHERE member_id = NEW.author_id) THEN
            RAISE EXCEPTION 'Invalid member_id for author_type member';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to enforce the author_type constraint
CREATE TRIGGER enforce_author_type_trigger_comments
BEFORE INSERT OR UPDATE ON comments
FOR EACH ROW EXECUTE FUNCTION enforce_author_type_comments();


-- Creating the 'friends' relationship table
CREATE TABLE friends (
    user_id1 INTEGER,
    user_id2 INTEGER,
    confirmed BOOLEAN DEFAULT FALSE,  -- To handle friend requests
    PRIMARY KEY (user_id1, user_id2),
    FOREIGN KEY (user_id1) REFERENCES member(member_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id2) REFERENCES member(member_id) ON DELETE CASCADE,
    CHECK (user_id1 <> user_id2)  -- Ensure a user cannot befriend themselves
);

-- Creating the 'block' table
CREATE TABLE block (
    blocker_id INTEGER,
    blocked_id INTEGER,
    PRIMARY KEY (blocker_id, blocked_id),
    FOREIGN KEY (blocker_id) REFERENCES member(member_id) ON DELETE CASCADE,
    FOREIGN KEY (blocked_id) REFERENCES member(member_id) ON DELETE CASCADE,
    CHECK (blocker_id <> blocked_id)  -- Ensure a user cannot block themselves
);

CREATE TABLE post_likes (
    user_id INTEGER,
    post_id INTEGER,
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES member(member_id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE
);

CREATE TABLE comment_likes (
    user_id INTEGER,
    comment_id INTEGER,
    PRIMARY KEY (user_id, comment_id),
    FOREIGN KEY (user_id) REFERENCES member(member_id) ON DELETE CASCADE,
    FOREIGN KEY (comment_id) REFERENCES comments(comment_id) ON DELETE CASCADE
);

-- Create the password_reset_tokens table
CREATE TABLE password_reset_tokens (
    token_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    reset_token VARCHAR(255) NOT NULL,
    reset_token_expires TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES member(member_id) ON DELETE CASCADE
);


-- Creating the 'user_authentication' view
CREATE VIEW user_authentication AS
SELECT
    member_id AS user_id,
    username,
    email,
    password_hash,
    full_name,
    phone_number,
    profile_pic,
    'member' AS user_type
FROM member
UNION ALL
SELECT
    admin_id AS user_id,
    username,
    NULL AS email,
    password_hash,
    full_name,
    phone_number,
    profile_pic,
    'admin' AS user_type
FROM admin;

