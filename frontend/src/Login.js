import React, { useState } from 'react';
import { useUserContext } from './App'; // Make sure the path is correct

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
    <div class="App">
      <form onSubmit={handleSubmit}>
        <label>
          Username:
          <input type="text" value={username} onChange={e => setUsername(e.target.value)} required />
        </label>
        <br />
        <label>
          Password:
          <input type="password" value={password} onChange={e => setPassword(e.target.value)} required />
        </label>
        <button type="submit">Login</button>
      </form>
    </div>
  );
}

export default Login;
