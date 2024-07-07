import React from 'react';
import { useNavigate } from 'react-router-dom';
import { useUserContext } from '../App'; // Adjust the import path according to your file structure
import '../assets/styles/HomePage.css'; // Import the CSS for styling

function HomePage() {
  const navigate = useNavigate();
  const [selectedPost, setSelectedPost] = React.useState(null);
  const { user } = useUserContext(); // Access the user context
  const posts = [
    { id: 1, title: "Post One", content: "Here is the first post's content...", likes: 10 },
    { id: 2, title: "Post Two", content: "Here is the second post's content...", likes: 20 },
    { id: 3, title: "Post Three", content: "Here is the third post's content...", likes: 30 },
    { id: 4, title: "Post Four", content: "Here is the fourth post's content...", likes: 40 },
    { id: 5, title: "Post Five", content: "Here is the fifth post's content...", likes: 50 }
  ];
  const showPostDetail = (post) => {
    setSelectedPost(post);
    navigate('/post-detail', { state: post });
  }

  return (
    <div className="homepage">
      {user ? <h1>Hi {user.fullname}</h1 > : <h1>Welcome</h1>
      }
      {posts.map(post => (
        <div key={post.id} className="post">
          <h2 onClick={() => showPostDetail(post)}>{post.title}</h2>
          <p>{post.content}</p>
          <div className="actions">
            <button>Like ({post.likes})</button>
            <button>Comment</button>
            <button>Share</button>
          </div>
        </div>
      ))}
    </div>);
}

export default HomePage;
