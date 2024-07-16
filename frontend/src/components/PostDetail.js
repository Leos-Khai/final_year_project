import React from 'react';
import { useLocation } from 'react-router-dom';
import '../assets/styles/PostDetail.css';

function PostDetail() {
  const { state: post } = useLocation();

  return (
    <div className="post-detail">
      <h1>{post.post_title}</h1>
      <p>{post.post_content}</p>
      <div className="likes">Likes: {post.like_count}</div>
      <div className="views">Views: {post.view_count}</div>
    </div>
  );
}

export default PostDetail;
