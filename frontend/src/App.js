import React, { createContext, useContext, useState, useMemo } from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import Login from './Login';
import HomePage from './HomePage';
import Logout from './Logout';
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
          <main>
            <Routes>
              <Route path="/login" element={!user ? <Login /> : <Navigate to="/" />} />
              <Route path="/" element={user ? <HomePage /> : <Navigate to="/login" />} />
              <Route path="/logout" element={<Logout />} />
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
