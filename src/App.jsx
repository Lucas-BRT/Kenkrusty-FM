import React from 'react';
import './styles/App.css';
import LoginPort from './pages/LoginPort';
import Soundboard from './pages/Soundboard';
import { Routes, Route } from 'react-router-dom'

function App() {
  return (
    <Routes>
      <Route exact path='/' Component={ LoginPort } />
      <Route path='/playlist' Component={ Soundboard }/>
    </Routes>
  );
}

export default App;
