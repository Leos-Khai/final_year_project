import React, { createContext, useContext, useState, useMemo } from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import Login from './components/Login';
import RegisterPage from './components/Registration';
import ForgotPassword from './components/ForgotPassword';
import ResetPassword from './components/ResetPassword';
import HomePage from './components/HomePage';
import Logout from './components/Logout';
import Post from './components/Post';
import Friends from './components/Friends';
import UserDetail from './components/UserDetail';
import PostDetail from './components/PostDetail';
import Header from './components/Header';
import Footer from './components/Footer';
import './App.css';

export const UserContext = createContext({ user: null, setUser: () => { } });

export function useUserContext() {
  return useContext(UserContext);
}

function App() {
  const [user, setUser] = useState(null);
  const providerValue = useMemo(() => ({ user, setUser }), [user]);

  return (
    <div className="App">
      <UserContext.Provider value={providerValue}>
        <Router>
          <Header />
          <main aria-live="polite">
            <Routes>
              <Route path="/login" element={!user ? <Login /> : <Navigate to="/" />} />
              <Route path="/register" element={!user ? <RegisterPage /> : <Navigate to="/" />} />
              <Route path="/forgot-password" element={!user ? <ForgotPassword /> : <Navigate to="/" />} />
              <Route path="/reset-password/:token" element={<ResetPassword />} />
              <Route path="/" element={user ? <HomePage /> : <Navigate to="/login" />} />
              <Route path="/logout" element={<Logout />} />
              <Route path="/post" element={user ? <Post /> : <Navigate to="/login" />} />
              <Route path="/friends" element={user ? <Friends /> : <Navigate to="/login" />} />
              <Route path="/user-detail" element={user ? <UserDetail /> : <Navigate to="/login" />} />
              <Route path="/post-detail/:id" element={user ? <PostDetail /> : <Navigate to="/login" />} />
              <Route path="/contact" element={<HomePage />} />
              <Route path="/about" element={<HomePage />} />
            </Routes>
          </main>
          <Footer />
        </Router>
      </UserContext.Provider>
    </div>
  );
}

export default App;
