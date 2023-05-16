import AppContext from './AppContext';
import { useState } from "react"

export default function Context({children}) {
  const [address, setAddress] = useState({
    ip: "127.0.0.1",
    port: "3333"
  })
  const [disable, setDisable] = useState(false)
  
  const obj = {
    address,
    setAddress,
    disable,
    setDisable,
  };

    return (
      <AppContext.Provider value={ obj } >
        {children}
      </AppContext.Provider>
    )
}