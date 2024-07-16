import React, { useState } from 'react';
import { createPost } from '../services/api';
import '../assets/styles/Post.css';

function Post() {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');

  const handleTitleChange = (e) => {
    setTitle(e.target.value);
  };

  const handleContentChange = (e) => {
    setContent(e.target.value);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    const newPost = {
      post_id: 0,
      post_title: title,
      post_content: content,
      like_count: 0,
      view_count: 0,
      author_type: "member",
      author_id: 0,
    };
    try {
      const response = await createPost(newPost);
      console.log('Post created:', response.data);
    } catch (error) {
      console.error('Error creating post:', error.response ? error.response.data : error.message);
    }
  };

  return (
    <div className="container">
      <div className="inputdiv">
        <input className="input" type="text" value={title} onChange={handleTitleChange} placeholder="Enter title" />
        <textarea className="textarea" value={content} onChange={handleContentChange} placeholder="Enter content" />
      </div>
      <button className="button" onClick={handleSubmit}>Submit</button>
    </div>
  );
}

export default Post;
