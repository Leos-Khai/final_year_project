import React from 'react';
import { useLocation } from 'react-router-dom';
import '../assets/styles/PostDetail.css';

function PostDetail() {
  const { state: post } = useLocation();

  return (
    <div className="post-detail">
      <h1>{post.title}</h1>
      <p>{post.content}</p>
      <div className="likes">Likes: {post.likes}</div>
    </div>
  );
}

export default PostDetail;
