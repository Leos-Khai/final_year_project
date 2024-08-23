import React, { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { resetPassword } from '../services/api';
import { validatePassword } from '../utils/passwordUtils'; // Import the validation function
import '../assets/styles/ResetPassword.css';

function ResetPassword() {
  const { token } = useParams();
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [error, setError] = useState('');
  const [validationError, setValidationError] = useState(null);
  const [success, setSuccess] = useState('');
  const navigate = useNavigate();

  const handleNewPasswordChange = (e) => {
    const value = e.target.value;
    setNewPassword(value);

    if (!validatePassword(value)) {
      setValidationError('Password must include lowercase, uppercase, number, and special character');
    } else {
      setValidationError(null);
    }
  };

  const handleSubmit = async (event) => {
    event.preventDefault();
    if (newPassword !== confirmPassword) {
      setError('Passwords do not match');
      return;
    }
    if (validationError) {
      setError(validationError);
      return;
    }
    try {
      const response = await resetPassword(token, newPassword);
      if (response.status === 200) {
        setSuccess('Password has been reset successfully');
        setTimeout(() => navigate('/login'), 3000); // Redirect to login page after 3 seconds
      } else {
        setError('Failed to reset password');
      }
    } catch (err) {
      setError(err.response?.data || 'Failed to reset password ');
    }
  };

  return (
    <div className="reset-password-container">
      <form onSubmit={handleSubmit}>
        <h2>Reset Password</h2>
        {error && <p style={{ color: 'red' }}>{error}</p>}
        {success && <p style={{ color: 'green' }}>{success}</p>}
        <label htmlFor="newPassword">New Password:</label>
        <input
          type="password"
          id="newPassword"
          value={newPassword}
          onChange={handleNewPasswordChange}
          required
        />
        {validationError && (
          <div className="validation-error">
            {validationError}
          </div>
        )}
        <label htmlFor="confirmPassword">Confirm Password:</label>
        <input
          type="password"
          id="confirmPassword"
          value={confirmPassword}
          onChange={(e) => setConfirmPassword(e.target.value)}
          required
        />
        <button type="submit">Reset Password</button>
      </form>
    </div>
  );
}

export default ResetPassword;
