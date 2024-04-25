-- Creating the 'member' table
CREATE TABLE member (
    memberId SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    passwordHash VARCHAR(255) NOT NULL,  -- Assuming hash storage for password
    fullName VARCHAR(255),
    phoneNumber VARCHAR(20),
    profilePic VARCHAR(255)
);

-- Creating the 'admin' table
CREATE TABLE admin (
    adminId SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    passwordHash VARCHAR(255) NOT NULL,
    fullName VARCHAR(255) NOT NULL,
    phoneNumber VARCHAR(20),
    adminLevel INTEGER CHECK (adminLevel BETWEEN 1 AND 3),
    profilePic VARCHAR(255)
);

-- Creating the 'post' table
CREATE TABLE post (
    postId SERIAL PRIMARY KEY,
    postTitle VARCHAR(255) NOT NULL,
    postContent TEXT NOT NULL,
    postDate DATE DEFAULT CURRENT_DATE,
    likeCount INTEGER DEFAULT 0,
    viewCount INTEGER DEFAULT 0,
    authorType VARCHAR(100) NOT NULL,
    authorId INTEGER NOT NULL,
    FOREIGN KEY (authorId) REFERENCES member(memberId) ON DELETE SET NULL
);

-- Creating the 'comments' table
CREATE TABLE comments (
    commentId SERIAL PRIMARY KEY,
    commentContent VARCHAR(255) NOT NULL,
    commentDate DATE DEFAULT CURRENT_DATE,
    authorType VARCHAR(100) NOT NULL,
    authorId INTEGER NOT NULL,
    postId INTEGER NOT NULL,
    FOREIGN KEY (postId) REFERENCES post(postId) ON DELETE CASCADE,
    FOREIGN KEY (authorId) REFERENCES member(memberId) ON DELETE SET NULL
);

-- Creating the 'friends' relationship table
CREATE TABLE friends (
    userId1 INTEGER,
    userId2 INTEGER,
    confirmed BOOLEAN DEFAULT FALSE,  -- To handle friend requests
    PRIMARY KEY (userId1, userId2),
    FOREIGN KEY (userId1) REFERENCES member(memberId) ON DELETE CASCADE,
    FOREIGN KEY (userId2) REFERENCES member(memberId) ON DELETE CASCADE,
    CHECK (userId1 <> userId2)  -- Ensure a user cannot befriend themselves
);

-- Creating the 'block' table
CREATE TABLE block (
    blockerId INTEGER,
    blockedId INTEGER,
    PRIMARY KEY (blockerId, blockedId),
    FOREIGN KEY (blockerId) REFERENCES member(memberId) ON DELETE CASCADE,
    FOREIGN KEY (blockedId) REFERENCES member(memberId) ON DELETE CASCADE,
    CHECK (blockerId <> blockedId)  -- Ensure a user cannot block themselves
);
