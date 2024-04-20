import React, { createContext, useContext, useState, useMemo } from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import Login from './Login'; // Import the Login component
import HomePage from './HomePage'; // A component you'll create for the home page after login
import './App.css';

export const UserContext = createContext({ user: null, setUser: () => { } });

export function useUserContext() {
  return useContext(UserContext);
}

function App() {
  const [user, setUser] = useState(null);

  const providerValue = useMemo(() => ({ user, setUser }), [user]);

  return (
    <UserContext.Provider value={providerValue}>
      <Router>
        <Routes>
          <Route path="/login" element={!user ? <Login /> : <Navigate to="/" />} />
          <Route path="/" element={user ? <HomePage /> : <Navigate to="/login" />} />
        </Routes>
      </Router>
    </UserContext.Provider>
    // Additional app component code...
  );
}

export default App;
