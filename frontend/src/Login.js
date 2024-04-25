import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { useUserContext } from './App'; // Make sure the path is correct
import './Login.css';

function Login() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const { setUser } = useUserContext();

  const handleSubmit = (event) => {
    event.preventDefault();
    console.log('Login attempt:', username, password);
    // Simulate authentication
    if (username === 'user' && password === 'pass') {
      setUser({
        username: username,
        fullname: 'John Doe'
      });
    } else {
      alert('Invalid credentials');
    }
  };

  return (
    <div className="login-container">
      <form onSubmit={handleSubmit}>
        <h2>Log In</h2>
        <label>
          Username:
          <input type="text" value={username} onChange={e => setUsername(e.target.value)} required />
        </label>
        <br />
        <label>
          Password:
          <input type="password" value={password} onChange={e => setPassword(e.target.value)} required />
        </label>
        <br />
        <p>Don't have an account? You can <Link to="/register">register here</Link>.</p>
        <input type="submit" value="Login" />
      </form>
    </div>
  );
}

export default Login;
