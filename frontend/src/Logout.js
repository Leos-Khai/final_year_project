import React from 'react';
import { useNavigate } from 'react-router-dom';
import { useUserContext } from './App';

function Logout() {
  const { setUser } = useUserContext();
  const navigate = useNavigate();

  const handleLogout = () => {
    setUser(null);
    navigate('/login');
  };

  return (
    <div>
      {handleLogout()}
    </div>
  );
}

export default Logout;