import React, { useState } from 'react';
import '../assets/styles/Post.css';

function Post({ post }) {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');

  const handleTitleChange = (e) => {
    setTitle(e.target.value);
  };

  const handleContentChange = (e) => {
    setContent(e.target.value);
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    // Handle post submission logic here
    // You can access the title and content using the state variables
    console.log('Title:', title);
    console.log('Content:', content);
  };

  return (
    <div className="container">
      <div className="inputdiv">
        <input className="input" type="text" value={title} onChange={handleTitleChange} placeholder="Enter title" />
        <textarea className="textarea" value={content} onChange={handleContentChange} placeholder="Enter content" />
      </div>
      <button className="button" onClick={handleSubmit}>Submit</button>
    </div >
  );
}

export default Post;