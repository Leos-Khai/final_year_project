import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';

function App() {
  const [counter, setCounter] = useState(0);
  // State to hold the list of messages
  const [messages, setMessages] = useState([]);

  // Function to handle button click
  const addMessage = () => {
    const newMessage = "Hi, world!"; // The message to add
    setMessages([...messages, newMessage]); // Add new message to the existing list
  };
  return (
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
  );
}

export default App;
