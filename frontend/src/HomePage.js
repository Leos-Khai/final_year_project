import React from 'react';
import { useUserContext } from './App'; // Adjust the import path according to your file structure

function HomePage() {
  const { user } = useUserContext(); // Access the user context

  return (
    <div>
      {user ? <h1>Hi {user.fullname}</h1> : <h1>Welcome</h1>}
    </div>
  );
}

export default HomePage;
