import React, { createContext, useContext, useState, useMemo } from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import Login from './Login';
import RegisterPage from './Registration';
import Register from './components/Register';
import HomePage from './HomePage';
import Logout from './Logout';
import Post from './Post';
import Friends from './Friends';
import UserDetail from './UserDetail';
import PostDetail from './PostDetail';
import Header from './Header';
import Footer from './Footer';
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
              <Route path="/register2" element={!user ? <Register /> : <Navigate to="/" />} />
              <Route path="/" element={user ? <HomePage /> : <Navigate to="/login" />} />
              <Route path="/logout" element={<Logout />} />
              <Route path="/post" element={user ? <Post /> : <Navigate to="/login" />} />
              <Route path="/friends" element={user ? <Friends /> : <Navigate to="/login" />} />
              <Route path="/user-detail" element={user ? <UserDetail /> : <Navigate to="/login" />} />
              <Route path="/post-detail" element={user ? <PostDetail /> : <Navigate to="/login" />} />
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
