import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { updatePassword } from '../services/api';
import { validatePassword } from '../utils/passwordUtils'; // Import the validation function
import '../assets/styles/UpdatePassword.css';

function UpdatePassword() {
  const navigate = useNavigate();
  const [oldPassword, setOldPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmNewPassword, setConfirmNewPassword] = useState('');
  const [error, setError] = useState(null);
  const [validationError, setValidationError] = useState(null);

  const isSubmitDisabled =
    oldPassword === newPassword ||
    newPassword !== confirmNewPassword ||
    !validatePassword(newPassword) || // Use the validation function
    !oldPassword ||
    !newPassword ||
    !confirmNewPassword;

  const handleNewPasswordChange = (e) => {
    const value = e.target.value;
    setNewPassword(value);

    if (!validatePassword(value)) {
      setValidationError('Password must include lowercase, uppercase, number, and special character');
    } else {
      setValidationError(null);
    }
  };

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
            onChange={handleNewPasswordChange}
            required
          />
        </div>
        {validationError && (
          <div className="validation-error">
            {validationError}
          </div>
        )}
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
