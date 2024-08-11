import axios from 'axios';

const API_URL = 'http://localhost:8080';

const axiosInstance = axios.create({
  baseURL: API_URL,
  withCredentials: true, // Important to include cookies in requests
});

// Auth API
export const register = async (username, email, password) => {
  return axiosInstance.post('/auth/register', { username, email, password });
};

export const login = async (username, password) => {
  return axiosInstance.post('/auth/login', { username, password });
};

export const logout = async () => {
  return axiosInstance.post('/auth/logout');
};

export const requestPasswordReset = async (email) => {
  return axiosInstance.post('/auth/request-reset', { email });
};

export const resetPassword = async (token, newPassword) => {
  return axiosInstance.post('/auth/reset-password', { token, new_password: newPassword });
};

// Posts API
export const createPost = async (post) => {
  return axiosInstance.post('/posts/create', post);
};

export const getPostById = async (postId) => {
  return axiosInstance.get(`/posts/${postId}`);
};

export const getPostsByUserId = async (userId) => {
  return axiosInstance.get(`/posts/user/${userId}`);
};

export const getPostsByFriends = async () => {
  return axiosInstance.get('/posts/friends');
};

export const getAllPosts = async () => {
  return axiosInstance.get('/posts/all');
};

export const updatePost = async (postId, post) => {
  return axiosInstance.put(`/posts/${postId}`, post);
};

export const deletePost = async (postId) => {
  return axiosInstance.delete(`/posts/${postId}`);
};

export const likePost = async (postId) => {
  return axiosInstance.post(`/posts/like/${postId}`);
};

export const checkPostValidity = async (postId) => {
  return axiosInstance.get(`/posts/check-validity/${postId}`);
};

// Comments API
export const createComment = async (postId, comment) => {
  return axiosInstance.post('/comments/create', { post_id: postId, comment_content: comment });
};

export const getCommentById = async (commentId) => {
  return axiosInstance.get(`/comments/${commentId}`);
};

export const getCommentsByPostId = async (postId) => {
  return axiosInstance.get(`/comments/post/${postId}`);
};

export const deleteComment = async (commentId) => {
  return axiosInstance.delete(`/comments/${commentId}`);
};
