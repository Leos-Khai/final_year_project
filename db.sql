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

-- Creating the 'post' table
CREATE TABLE post (
    post_id SERIAL PRIMARY KEY,
    post_title VARCHAR(255) NOT NULL,
    post_content TEXT NOT NULL,
    post_date DATE DEFAULT CURRENT_DATE,
    like_count INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    author_type VARCHAR(100) NOT NULL,
    author_id INTEGER NOT NULL,
    FOREIGN KEY (author_id) REFERENCES member(member_id) ON DELETE SET NULL
        DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (author_id) REFERENCES admin(admin_id) ON DELETE SET NULL
        DEFERRABLE INITIALLY DEFERRED
);

-- Creating the 'comments' table
CREATE TABLE comments (
    comment_id SERIAL PRIMARY KEY,
    comment_content VARCHAR(255) NOT NULL,
    comment_date DATE DEFAULT CURRENT_DATE,
    author_type VARCHAR(100) NOT NULL,
    author_id INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    FOREIGN KEY (post_id) REFERENCES post(post_id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES member(member_id) ON DELETE SET NULL
        DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (author_id) REFERENCES admin(admin_id) ON DELETE SET NULL
        DEFERRABLE INITIALLY DEFERRED
);

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
