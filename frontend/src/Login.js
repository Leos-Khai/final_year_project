import React, { useState } from 'react';
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
        <input type="submit" value="Login" />
      </form>
    </div>
  );
}

export default Login;
