import React from 'react';
import { useUserContext } from './App'; // Adjust the import path according to your file structure

function HomePage() {
  const { user } = useUserContext(); // Access the user context

  return (
    <div>
      {user ? <h1>Hi {user.fullname}</h1> : <h1>Welcome</h1>}
      <p>This is a really long paragraph talking about absolutely nothing. the main purpose of this is there is none. First lets talk about the uselessness of this long and senseless paragraph. it surves no purpose than just be blabla thing on the screen. enorder to make this test happen i need a really really long amount of useless text.</p>
    </div>
  );
}

export default HomePage;
