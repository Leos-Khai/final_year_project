import React, { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useUserContext } from '../App'; // Adjust the path if necessary
import { login } from '../services/api'; // Import the login function from api.js
import '../assets/styles/Login.css';

function Login() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const { setUser } = useUserContext(); // Access the user context
  const [error, setError] = useState(null); // Initialize error state as null
  const navigate = useNavigate(); // Hook to navigate to different routes

  const handleSubmit = async (event) => {
    event.preventDefault(); // Prevent default form submission
    try {
      const response = await login(username, password); // Attempt to login
      if (response.status === 200) {
        const user = response.data;
        console.log('Logged in user:', user);
        setUser(user); // Set the user in context
        navigate('/'); // Redirect to homepage or dashboard after successful login
      } else {
        setError('Invalid credentials');
      }
    } catch (err) {
      setError(err.response?.data || 'Invalid credentials');
    }
  };

  return (
    <div className="login-container">
      <form onSubmit={handleSubmit}>
        <h2>Log In</h2>
        {error && <p className="error-message">{error}</p>} {/* Display error message if any */}
        <div className="form-group">
          <label htmlFor="username">Username:</label>
          <input
            type="text"
            id="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="password">Password:</label>
          <input
            type="password"
            id="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
        </div>
        <button type="submit" className="login-button">Login</button>
        <p className="link-text">
          Don't have an account? <Link to="/register">Register here</Link>.
        </p>
        <p className="link-text">
          Forgot your password? <Link to="/forgot-password">Reset it here</Link>.
        </p>
      </form>
    </div>
  );
}

export default Login;
