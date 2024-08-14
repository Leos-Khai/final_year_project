import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import '../assets/styles/PostDetail.css';
import { getPostById, checkPostValidity, deletePost, likePost } from '../services/api';
import { useUserContext } from '../App';
import CommentSection from './CommentSection'; // Import the CommentSection component

function PostDetail() {
  const { id } = useParams(); // Get the post ID from the URL
  const navigate = useNavigate();
  const { user } = useUserContext(); // Access the user context
  const [post, setPost] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [validity, setValidity] = useState(null);
  const [showModal, setShowModal] = useState(false); // State for controlling the modal

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

  const handleLike = async () => {
    try {
      const response = await likePost(id); // Call the likePost API
      setPost(response.data); // Update the post data with the new like count
    } catch (error) {
      console.error('Error liking post:', error);
      setError('Error liking post');
    }
  };

  const handleCheckValidity = async () => {
    try {
      const response = await checkPostValidity(id);
      setValidity(response.data);
      setShowModal(true); // Show the modal with the validity results
    } catch (error) {
      console.error('Error checking post validity:', error);
      setError('Error checking post validity');
    }
  };

  const closeModal = () => {
    setShowModal(false); // Hide the modal
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
        <button className="likes-button" onClick={handleLike}>Likes: {post.like_count}</button>
        {user && user.user_id === post.author_id && (
          <div className="author-actions">
            <button className="update-button" onClick={handleUpdate}>Update</button>
            <button className="delete-button" onClick={handleDelete}>Delete</button>
          </div>
        )}
        <br />
        <br />
        <button className="validity-button" onClick={handleCheckValidity}>Check Post Validity</button>
      </div>

      {/* Add the CommentSection here */}
      <CommentSection postId={id} />

      {/* Modal for displaying validity results */}
      {
        showModal && (
          <div className="modal" style={{ display: 'block' }}>
            <div className="modal-content">
              <span className="close" onClick={closeModal}>&times;</span>
              <h2>Post Validity Result</h2>
              <p>Validity: {validity.result}</p>
              <p>Fake News Probability: {validity.fake_probability * 100}%</p>
              <p>Real News Probability: {validity.real_probability * 100}%</p>
            </div>
          </div>
        )
      }
    </div >
  );
}

export default PostDetail;
