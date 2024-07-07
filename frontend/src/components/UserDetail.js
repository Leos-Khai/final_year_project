import React from 'react';
import { useLocation } from 'react-router-dom';
import './UserDetail.css';

function UserDetail({ user }) {
  const location = useLocation();
  const friend = location.state; // Access the passed state
  return (
    <div className="user-detail-container">
      <h1>User Profile</h1>
      {friend ? (

        <div className="user-info">
          <p><strong>Name:</strong> {friend.name}</p>
          <p><strong>Age:</strong> {friend.age}</p>
          <p><strong>Occupation:</strong> {friend.occupation}</p>
        </div>
          ) : (
          <p>No user data available.</p>
      )}
    </div>
  );
}

export default UserDetail;
