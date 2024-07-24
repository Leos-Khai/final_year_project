import React, { useState, useEffect } from 'react';
import '../assets/styles/Registration.css';
import { register } from '../services/api';

function RegistrationPage() {
  const [formData, setFormData] = useState({
    username: '',
    email: '',
    password: '',
    confirmPassword: ''
  });
  const [error, setError] = useState(null);
  const [isButtonDisabled, setIsButtonDisabled] = useState(true);

  useEffect(() => {
    const { password, confirmPassword } = formData;
    setIsButtonDisabled(password !== confirmPassword || !password || !confirmPassword);
  }, [formData]);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData(prevState => ({
      ...prevState,
      [name]: value
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (formData.password !== formData.confirmPassword) {
      alert("Passwords do not match!");
      return;
    }
    try {
      await register(formData.username, formData.email, formData.password);
      alert('User registered');
    } catch (error) {
      console.error('Error registering user:', error);
      setError(error.response?.data || 'Error registering user');
    }
  };

  return (
    <form className="registration-form" onSubmit={handleSubmit}>
      <h1>Register</h1>
      <label htmlFor="username">Username:</label>
      <input
        type="text"
        id="username"
        name="username"
        value={formData.username}
        onChange={handleChange}
      />

      <label htmlFor="email">Email:</label>
      <input
        type="email"
        id="email"
        name="email"
        value={formData.email}
        onChange={handleChange}
      />

      <label htmlFor="password">Password:</label>
      <input
        type="password"
        id="password"
        name="password"
        value={formData.password}
        onChange={handleChange}
      />

      <label htmlFor="confirmPassword">Confirm Password:</label>
      <input
        type="password"
        id="confirmPassword"
        name="confirmPassword"
        value={formData.confirmPassword}
        onChange={handleChange}
      />

      <button type="submit" disabled={isButtonDisabled}>Register</button>
      {error && <div style={{ color: 'red' }}>{error}</div>}
    </form>
  );
}

export default RegistrationPage;
