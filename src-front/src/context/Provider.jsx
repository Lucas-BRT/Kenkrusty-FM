import AppContext from './AppContext';
import React, { useState } from "react"

export default function Context({children}) {
  const [address, setAddress] = useState({
    ip: "127.0.0.1",
    port: "3333"
  });
  const [disable, setDisable] = useState(false);
  const [playlist, setPlaylist] = useState([]);
  const [sounds, setSounds] = useState([]);
  
  const obj = {
    address,
    setAddress,
    disable,
    setDisable,
    playlist,
    setPlaylist,
    sounds,
    setSounds,
  };

    return (
      <AppContext.Provider value={ obj } >
        {children}
      </AppContext.Provider>
    )
}