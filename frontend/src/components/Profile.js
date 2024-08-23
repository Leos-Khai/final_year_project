import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getProfile, deleteProfile, logout } from '../services/api'; // Import logout function
import '../assets/styles/Profile.css';
import { useUserContext } from '../App';

function Profile() {
  const { setUser } = useUserContext();
  const [profile, setProfile] = useState(null);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    const fetchProfile = async () => {
      try {
        const response = await getProfile();
        setProfile(response.data);
      } catch (error) {
        setError(error.response?.data || 'Error fetching profile information.');
        console.error('Error fetching profile:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchProfile();
  }, []);

  const handleUpdate = () => {
    navigate('/update-profile');
  };

  const handleUpdatePassword = () => {
    navigate('/update-password');
  };

  const handleDelete = async () => {
    if (window.confirm('Are you sure you want to delete your profile?')) {
      try {
        await deleteProfile();  // Delete profile on the server
        await logout();  // Clear session on the server
        setUser(null);
        navigate('/login');  // Redirect to login page
      } catch (error) {
        setError(error.response?.data || 'Error deleting profile.');
        console.error('Error deleting profile:', error);
      }
    }
  };

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>{error}</div>;
  }

  if (!profile) {
    return <div>No profile information available.</div>;
  }

  return (
    <div className="profile-container">
      <h1>Profile Information</h1>
      <div className="profile-info">
        <p><strong>Username:</strong> {profile.username}</p>
        <p><strong>Email:</strong> {profile.email}</p>
        <p><strong>Full Name:</strong> {profile.full_name || 'N/A'}</p>
        <p><strong>Phone Number:</strong> {profile.phone_number || 'N/A'}</p>
      </div>
      <div className="profile-actions">
        <button onClick={handleUpdate} className="update-button">Update Profile</button>
        <button onClick={handleUpdatePassword} className="update-password-button">Update Password</button>
        <button onClick={handleDelete} className="delete-button">Delete Profile</button>
      </div>
    </div>
  );
}

export default Profile;
/* <p><strong>Profile Picture:</strong> {profile.profile_pic ? <img src={profile.profile_pic} alt="Profile" /> : 'N/A'}</p> */
