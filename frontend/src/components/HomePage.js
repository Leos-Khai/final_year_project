import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useUserContext } from '../App'; // Adjust the import path according to your file structure
import '../assets/styles/HomePage.css'; // Import the CSS for styling
import { getAllPosts } from '../services/api'; // Import the API call

function HomePage() {
  const navigate = useNavigate();
  const [posts, setPosts] = useState([]);
  const { user } = useUserContext(); // Access the user context

  // Fetch posts from API
  useEffect(() => {
    const fetchPosts = async () => {
      try {
        const response = await getAllPosts();
        console.log("Fetched posts:", response.data); // Add this line to check the response
        setPosts(response.data);
      } catch (error) {
        console.error("Error fetching posts:", error);
      }
    };

    fetchPosts();
  }, []);

  const showPostDetail = (post) => {
    navigate(`/post-detail/${post.post_id}`);
  };

  return (
    <div className="homepage">
      {user ? <h1>Hi {user.username}</h1> : <h1>Welcome</h1>}
      {posts.map(post => (
        <div key={post.post_id} className="post">
          <h2 onClick={() => showPostDetail(post)}>{post.post_title}</h2> {/* Use post_title */}
          <p>{post.post_content}</p> {/* Use post_content */}
          <div className="actions">
            <button>Like ({post.like_count})</button> {/* Use like_count */}
            <button>Comment</button>
            <button>Share</button>
          </div>
        </div>
      ))}
    </div>
  );
}

export default HomePage;
