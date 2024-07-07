import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useUserContext } from '../App';
import { logout as apiLogout } from '../services/api'; // Make sure the path is correct

function Logout() {
  const { setUser } = useUserContext();
  const navigate = useNavigate();

  useEffect(() => {
    const handleLogout = async () => {
      try {
        await apiLogout();
        setUser(null);
        navigate('/login');
      } catch (error) {
        console.error('Logout failed', error);
      }
    };

    handleLogout();
  }, [setUser, navigate]);

  return <div>Logging out...</div>;
}

export default Logout;
