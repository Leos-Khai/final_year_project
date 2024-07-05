import axios from 'axios';

const API_URL = 'http://localhost:8080';

export const register = async (username, email, password) => {
  return axios.post(`${API_URL}/auth/register`, { username, email, password });
};

export const login = async (username, password) => {
  return axios.post(`${API_URL}/auth/login`, { username, password });
};
