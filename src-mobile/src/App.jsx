import React, { useEffect, useState } from 'react';
import { ErrorPage } from './pages/errorPage/ErrorPage';
import { LoadingPage } from './pages/loadingPage/LoadingPage';
import MainPage from './pages/mainPage/MainPage'


function App() {
  const [kenkuViability, setKenkuViability] = useState(null);
  
  useEffect(() => {
    checkKenkuFMViability();
  }, []);
  
  async function checkKenkuFMViability() {
    try {
      const response = await fetch('/kenkuFMState');
      const data = await response.json();
  
      setKenkuViability(data);
    } catch (error) {

    }
  }
  return (

    <MainPage />

  )

  /*

  if (kenkuViability === null) {
    // Renderiza um componente de carregamento ou uma mensagem enquanto os dados são buscados
    return (
      <>
        <LoadingPage/>
      </>
    )
  } else if (kenkuViability == '200') {
    return (
      <>
        <MainPage />
      </>
    )        
  } else if (kenkuViability == '404' || kenkuViability == null) {
    return (
      <>
        <ErrorPage error="macacos invadiram seu computador!" />
      </>
    );
  }

  */

}

export default App;
