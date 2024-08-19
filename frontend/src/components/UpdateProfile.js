import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getProfile, updateProfile } from '../services/api';
import '../assets/styles/UpdateProfile.css';

function UpdateProfile() {
  const navigate = useNavigate();
  const [profile, setProfile] = useState({
    member_id: '',
    username: '',
    email: '',
    full_name: '',
    phone_number: '',
    profile_pic: '',
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchProfile = async () => {
      try {
        const response = await getProfile();
        setProfile({
          member_id: response.data.member_id,
          username: response.data.username,
          email: response.data.email,
          full_name: response.data.full_name || '',
          phone_number: response.data.phone_number || '',
          profile_pic: response.data.profile_pic || '',
        });
      } catch (error) {
        setError('Error fetching profile details.');
        console.error('Error fetching profile:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchProfile();
  }, []);

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      // Send the full profile object, including both modified and unmodified
      // values
      const jsonString = JSON.stringify(profile, null, 2);
      navigator.clipboard.writeText(jsonString);
      await updateProfile(profile);
      navigate('/profile');
    } catch (error) {
      setError('Error updating profile.');
      console.error('Error updating profile:', error);
    }
  };

  const handleChange = (e) => {
    const { name, value } = e.target;
    setProfile({
      ...profile,
      [name]: value,
    });
  };

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>{error}</div>;
  }

  return (
    <div className="update-profile">
      <h1>Update Profile</h1>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="username">Username</label>
          <input
            type="text"
            id="username"
            name="username"
            value={profile.username}
            disabled
          />
        </div>
        <div className="form-group">
          <label htmlFor="email">Email</label>
          <input
            type="email"
            id="email"
            name="email"
            value={profile.email}
            disabled
          />
        </div>
        <div className="form-group">
          <label htmlFor="full_name">Full Name</label>
          <input
            type="text"
            id="full_name"
            name="full_name"
            value={profile.full_name}
            onChange={handleChange}
          />
        </div>
        <div className="form-group">
          <label htmlFor="phone_number">Phone Number</label>
          <input
            type="text"
            id="phone_number"
            name="phone_number"
            value={profile.phone_number}
            onChange={handleChange}
          />
        </div>
        <button type="submit">Update Profile</button>
      </form>
    </div>
  );
}

export default UpdateProfile;
