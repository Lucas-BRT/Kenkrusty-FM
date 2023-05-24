import React from 'react';
import './App.css';
import { useContext, useEffect } from 'react';
import AppContext from './context/AppContext';

function Form() {
  const {address, setAddress, disable, setDisable} = useContext(AppContext)
  console.log(disable)
  const handleInput = ({target}) => {
    const { value, name } = target
    setAddress((prevState) => ({
      ...prevState,
      [name]: value
    }))
  }

  useEffect(() => {
    const regex = /^(\d{1,3}\.){3}\d{1,3}$/;
    if (!regex.test(address.ip)) {
      setDisable(true)
    } else {
      setDisable(false)
    }
  }, [address.ip, setDisable])

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
            />
        </div>
    </div>
  )
}

export default Form;