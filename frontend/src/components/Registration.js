import React, { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
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
  const [success, setSuccess] = useState(null);
  const [isButtonDisabled, setIsButtonDisabled] = useState(true);
  const navigate = useNavigate();

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
      setError("Passwords do not match!");
      return;
    }
    try {
      await register(formData.username, formData.email, formData.password);
      setSuccess('User registered successfully. Redirecting to login...');
      setTimeout(() => {
        navigate('/login');
      }, 5000);
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
        required
        id="username"
        name="username"
        value={formData.username}
        onChange={handleChange}
      />

      <label htmlFor="email">Email:</label>
      <input
        required
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
      {success && <div style={{ color: 'green' }}>{success}</div>}
      <p>Already have an account? <Link to="/login">Login here</Link>.</p>
    </form>
  );
}

export default RegistrationPage;
