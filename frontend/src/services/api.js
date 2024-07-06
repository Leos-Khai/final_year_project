import axios from 'axios';

const API_URL = 'http://localhost:8080';

const axiosInstance = axios.create({
  baseURL: API_URL,
  withCredentials: true, // Important to include cookies in requests
});

export const register = async (username, email, password) => {
  return axiosInstance.post('/auth/register', { username, email, password });
};

export const login = async (username, password) => {
  return axiosInstance.post('/auth/login', { username, password });
};

export const logout = async () => {
  return axiosInstance.post('/auth/logout');
};

export const getProtected = async () => {
  return axiosInstance.get('/protected');
};
