import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { useUserContext } from '../App'; // Make sure the path is correct
import { login } from '../services/api'; // Import the login function from api.js
import '../assets/styles/Login.css';

function Login() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const { setUser } = useUserContext();
  const [error, setError] = useState('');

  const handleSubmit = async (event) => {
    event.preventDefault();
    try {
      const response = await login(username, password);
      if (response.status === 200) {
        const user = response.data;
        console.log('Logged in user:', user);
        setUser(user); // Set user context with the logged-in user's info
      } else {
        setError('Invalid credentials');
      }
    } catch (err) {
      setError('Invalid credentials');
    }
  };

  return (
    <div className="login-container">
      <form onSubmit={handleSubmit}>
        <h2>Log In</h2>
        {error && <p style={{ color: 'red' }}>{error}</p>}
        <label htmlFor="username">Username:</label>
        <input
          type="text"
          id="username"
          value={username}
          onChange={e => setUsername(e.target.value)}
          required
        />
        <label htmlFor="password">Password:</label>
        <input
          type="password"
          id="password"
          value={password}
          onChange={e => setPassword(e.target.value)}
          required
        />
        <p>Don't have an account? You can <Link to="/register">register here</Link>.</p>
        <button type="submit">Login</button>
      </form>
    </div>
  );
}

export default Login;
