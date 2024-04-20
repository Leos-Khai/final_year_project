import React, { useState, useMemo } from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import Login from './Login'; // Import the Login component
import HomePage from './HomePage'; // A component you'll create for the home page after login
import logo from './logo.svg';
import './App.css';

export const UserContext = React.createContext(null);

function App() {
  // login section
  const [user, setUser] = useState(null); // State to hold the user object
  const providerValue = useMemo(() => ({ user, setUser }), [user, setUser]); // Memoize the context value to avoid unnecessary re-renders

  // counter example.
  const [counter, setCounter] = useState(0);
  // State to hold the list of messages
  const [messages, setMessages] = useState([]);

  // Function to handle button click
  const addMessage = () => {
    const newMessage = "Hi, world!"; // The message to add
    setMessages([...messages, newMessage]); // Add new message to the existing list
  };

  return (
    <div>
      <UserContext.Provider value={providerValue}>
        <Router>
          <Routes>
            <Route path="/login" element={user ? <Navigate to="/" /> : <Login onLogin={setUser} />} />
            <Route path="/" element={user ? <HomePage /> : <Navigate to="/login" />} />
          </Routes>
        </Router>
      </UserContext.Provider>
      <div className="App">
        <header className="App-header">
          <p aria-live="assertive">You clicked {counter} times</p>
          <button onClick={() => setCounter(counter + 1)}>Click me</button>
          <br />
          <div>
            <button onClick={addMessage}>Add Message</button>
            <div aria-live="polite">
              {messages.map((message, index) => (
                <div key={index}>{message}</div> // Display each message on a new line
              ))}
            </div>
          </div>
          <img src={logo} className="App-logo" alt="logo" />
          <p>
            Edit <code>src/App.js</code> and save to reload.
          </p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
        </header>
      </div>
    </div>
  );
}

export default App;
