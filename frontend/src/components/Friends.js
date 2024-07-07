import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import '../assets/styles/Friends.css';

function Friends() {
  const navigate = useNavigate();
  const [selectedFriend, setSelectedFriend] = useState(null);
  const friendsList = [
    { id: 1, name: "Alice", age: 25, occupation: "Engineer" },
    { id: 2, name: "Bob", age: 28, occupation: "Designer" },
    { id: 3, name: "Charlie", age: 30, occupation: "Chef" },
    { id: 4, name: "Diana", age: 24, occupation: "Photographer" },
    { id: 5, name: "Evan", age: 29, occupation: "Musician" }
  ];

  const handleFriendClick = (friend) => {
    setSelectedFriend(friend);
    navigate('/user-detail', { state: friend });
  };
  return (
    <div className="friends-container">
      <h1>My Friends</h1>
      <ul>
        {friendsList.map(friend => (
          <li key={friend.id} className="friend-item" onClick={() => handleFriendClick(friend)}>
            {friend.name}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Friends;
