import React from 'react';
import './App.css';
import { useContext, useEffect } from 'react';
import AppContext from './context/AppContext';

function Form() {
  const {address, setAddress, disable, setDisable} = useContext(AppContext)
  const handleInput = ({target}) => {
    const { value, name } = target
    setAddress((prevState) => ({
      ...prevState,
      [name]: value
    }))
  }

  class warningEntry {
    constructor(identifier) {
      this.identifier = identifier
    }

    raiseRedWarning() {
      // tornar o entry point em vermelho para chamar a atenção do usuário para aquele campo
      // informando que é um erro que impede a conecção
    }

    raiseYellowWarning() {
      // tornar o entry point em amarelo para chamar a atenção do usuário para aquele campo
      // nesse caso é para ocasiões como deixar a porta em branco
      // apesar de já haver uma checagem para garantir a execução, pessoas menos atentas
      // podem acabar não lembrando da porta caso a porta 0 não seja a da conexão
    }
  }

  class ValidateConection {
    constructor(ip,port) {
      this.ip = ip;
      this.port = port;
      if (port == "") {
        this.port = 0
      }
    }
    
    validateConection() {
      return (this.validateIP() && this.validatePort())
    }

    validateIP() {
      const ipRegex = /^(\d{1,3}\.){3}\d{1,3}$/;
      return ipRegex.test(this.ip)
    }

    validatePort() {
      const portRegex = /^(0|6553[0-5]|655[0-2]\d|65[0-4]\d{2}|6[0-4]\d{3}|[1-5]\d{4}|\d{1,4})$/;
      return portRegex.test(this.port)
    }
  }

  useEffect(() => {
    const conection = new ValidateConection(address.ip,address.port)
    const allowConection = conection.validateConection()

    if (allowConection) {
      setDisable(false)
    } else {
      if (!conection.validateIP()) {
        setDisable(true)
        // warning red
      } else {
        setDisable(true)
        // warning yellow
      }
    }
  
  }, [address.ip,address.port,setDisable])

  return (
    <div className="view">
        <div className="card">
            <label htmlFor="ip" id='label'>
              IP:
              <input
                type="text"
                id="ip"
                value={ address.ip }
                name="ip"
                onChange={ handleInput }
              />
            </label>
            <label htmlFor="port" id='label'>
              Port:
              <input
                type="number"
                id="port"
                value={ address.port }
                name="port"
                onChange={ handleInput }
              />
            </label>
            <input
              type="submit"
              value="Connect"
              id="button"
              disabled={ disable }
              onClick={ () => {
                const { invoke } = window.__TAURI__.tauri
                invoke("conection_test", { ip: address.ip, port: address.port})
                  .then(
                    (response) => {
                      console.log(response)
                    }
                )

                }
              }
            />
        </div>
    </div>
  )
}

export default Form;