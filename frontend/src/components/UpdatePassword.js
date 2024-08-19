import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { updatePassword } from '../services/api';
import '../assets/styles/UpdatePassword.css';

function UpdatePassword() {
  const navigate = useNavigate();
  const [oldPassword, setOldPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmNewPassword, setConfirmNewPassword] = useState('');
  const [error, setError] = useState(null);

  const isSubmitDisabled =
    oldPassword === newPassword ||
    newPassword !== confirmNewPassword ||
    !oldPassword ||
    !newPassword ||
    !confirmNewPassword;

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await updatePassword(oldPassword, newPassword);
      navigate('/profile');
    } catch (error) {
      setError('Error updating password.');
      console.error('Error updating password:', error);
    }
  };

  return (
    <div className="update-password">
      <h1>Update Password</h1>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="old_password">Old Password</label>
          <input
            type="password"
            id="old_password"
            name="old_password"
            value={oldPassword}
            onChange={(e) => setOldPassword(e.target.value)}
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="new_password">New Password</label>
          <input
            type="password"
            id="new_password"
            name="new_password"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="confirm_new_password">Confirm New Password</label>
          <input
            type="password"
            id="confirm_new_password"
            name="confirm_new_password"
            value={confirmNewPassword}
            onChange={(e) => setConfirmNewPassword(e.target.value)}
            required
          />
        </div>
        <button type="submit" disabled={isSubmitDisabled}>
          Update Password
        </button>
        {error && <div className="error-message">{error}</div>}
      </form>
    </div>
  );
}

export default UpdatePassword;
