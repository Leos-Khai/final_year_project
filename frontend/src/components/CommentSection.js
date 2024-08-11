import React, { useState, useEffect } from 'react';
import { getCommentsByPostId, createComment } from '../services/api';
import { useUserContext } from '../App';
import '../assets/styles/CommentSection.css';

function CommentSection({ postId }) {
  const { user } = useUserContext();
  const [comments, setComments] = useState([]);
  const [newComment, setNewComment] = useState('');
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchComments = async () => {
      try {
        const response = await getCommentsByPostId(postId);
        setComments(response.data);
      } catch (error) {
        setError('Error fetching comments.');
        console.error('Error fetching comments:', error);
      }
    };

    fetchComments();
  }, [postId]);

  const handleCommentSubmit = async (e) => {
    e.preventDefault();

    if (!newComment.trim()) return;

    try {
      const response = await createComment(parseInt(postId, 10), newComment); // Separate arguments
      setComments([...comments, response.data]);
      setNewComment('');
    } catch (error) {
      setError('Error adding comment.');
      console.error('Error adding comment:', error);
    }
  };

  return (
    <div className="comment-section">
      <h3>Comments</h3>
      {error && <div className="error">{error}</div>}
      <ul className="comment-list">
        {comments.map((comment) => (
          <li key={comment.comment_id}>
            <strong>{comment.author_name}</strong>:
            <span>{comment.comment_content}</span>
          </li>
        ))}
      </ul>

      {user && (
        <form onSubmit={handleCommentSubmit} className="comment-form">
          <textarea
            value={newComment}
            onChange={(e) => setNewComment(e.target.value)}
            placeholder="Add a comment..."
          ></textarea>
          <button type="submit">Submit</button>
        </form>
      )
      }
    </div >
  );
}

export default CommentSection;
