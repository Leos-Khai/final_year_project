import React, { useState } from 'react';
import { requestPasswordReset } from '../services/api'; // Import the password reset request function from api.js
import '../assets/styles/ForgotPassword.css';

function ForgotPassword() {
  const [email, setEmail] = useState('');
  const [message, setMessage] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = async (event) => {
    event.preventDefault();
    try {
      const response = await requestPasswordReset(email);
      if (response.status === 200) {
        setMessage('Password reset email sent. Please check your inbox.');
        setError('');
      } else {
        setMessage('');
        setError('Error sending password reset email.');
      }
    } catch (err) {
      setMessage('');
      setError('Error sending password reset email.');
    }
  };

  return (
    <div className="forgot-password-container">
      <form onSubmit={handleSubmit}>
        <h2>Forgot Password</h2>
        {message && <p style={{ color: 'green' }}>{message}</p>}
        {error && <p style={{ color: 'red' }}>{error}</p>}
        <label htmlFor="email">Email:</label>
        <input
          type="email"
          id="email"
          value={email}
          onChange={e => setEmail(e.target.value)}
          required
        />
        <button type="submit">Send Reset Link</button>
      </form>
    </div>
  );
}

export default ForgotPassword;
