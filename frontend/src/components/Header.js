import React from 'react';
import { Link } from 'react-router-dom';
import { useUserContext } from '../App';
import '../assets/styles/Header.css';

function Header() {
  const { user } = useUserContext();
  return (
    <header className="header">
      <div className="logo">Social Media Sdn. Bhd.</div>
      <nav>
        <Link to="/">Home</Link>
        <Link to="/about">About</Link>
        <Link to="/contact">Contact</Link>
        {user && <Link to="/friends">Friends</Link>}
        {user && <Link to="/post">Post</Link>}
        {user && <Link to="/profile">Profile</Link>}
        {!user ? (<Link to="/login">Log-in</Link>) : (<Link to="/logout">Log-out</Link>)}
        {!user && <Link to="/register">Register</Link>}
      </nav>
    </header>
  );
}

export default Header;
