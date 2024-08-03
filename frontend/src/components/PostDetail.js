import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import '../assets/styles/PostDetail.css';
import { getPostById, checkPostValidity, deletePost } from '../services/api';
import { useUserContext } from '../App'; // Adjust the import path according to your file structure

function PostDetail() {
  const { id } = useParams(); // Get the post ID from the URL
  const navigate = useNavigate();
  const { user } = useUserContext(); // Access the user context
  const [post, setPost] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [validity, setValidity] = useState(null);

  useEffect(() => {
    const fetchPost = async () => {
      try {
        const response = await getPostById(id);
        setPost(response.data);
      } catch (error) {
        setError("Error fetching post details.");
        console.error("Error fetching post:", error);
      } finally {
        setLoading(false);
      }
    };

    fetchPost();
  }, [id]);

  const handleUpdate = () => {
    navigate(`/update-post/${id}`);
  };

  const handleDelete = async () => {
    try {
      await deletePost(id);
      navigate('/');
    } catch (error) {
      console.error('Error deleting post:', error);
      setError('Error deleting post');
    }
  };

  const handleCheckValidity = async () => {
    try {
      const response = await checkPostValidity(id);
      setValidity(response.data);
    } catch (error) {
      console.error('Error checking post validity:', error);
      setError('Error checking post validity');
    }
  };

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>{error}</div>;
  }

  if (!post) {
    return <div>No post found</div>;
  }

  return (
    <div className="post-detail-container">
      <div className="post-detail">
        <h1>{post.post_title}</h1>
        <div className="views">Views: {post.view_count}</div>
        <p>{post.post_content}</p>
        <button className="likes-button">Likes: {post.like_count}</button>
        {user && user.user_id === post.author_id && (
          <div className="author-actions">
            <button className="update-button" onClick={handleUpdate}>Update</button>
            <button className="delete-button" onClick={handleDelete}>Delete</button>
          </div>
        )}
        <button className="validity-button" onClick={handleCheckValidity}>Check Post Validity</button>
        {validity && <div className="validity-result">Validity: {validity}</div>}
      </div>
    </div>
  );
}

export default PostDetail;
