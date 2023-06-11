import React from "react";
import { useContext, useEffect } from "react";
import AppContext from "../context/AppContext";

function LoginPort() {
  const {
    address,
    setAddress,
    disable,
    setDisable,
  } = useContext(AppContext);

  const { invoke } = window.__TAURI__.tauri;

  const handleInput = ({ target }) => {
    const { value, name } = target;
    setAddress((prevState) => ({
      ...prevState,
      [name]: value,
    }));
  };

  const checkAddress = async () => {
    await invoke("is_kenku_remote_avaliable", { ip: address.ip, port: address.port })
      .then((res) => {
        return res;
      })
      .catch((err) => {
        console.error(err);
      });
  };

  const connectPort = async () => {

    await invoke("connect", {ip: address.ip , port:address.port});
  
  };

  useEffect(() => {
    const regex = /^(\d{1,3}\.){3}\d{1,3}$/;
    if (!regex.test(address.ip)) {
      setDisable(true);
    } else {
      setDisable(false);
    }
  }, [address.ip, setDisable]);

  return (
    <div className="view">
      <div className="card">
        <label htmlFor="ip" id="label">
          IP:
          <input
            type="text"
            id="ip"
            value={address.ip}
            name="ip"
            onChange={handleInput}
          />
        </label>
        <label htmlFor="port" id="label">
          Port:
          <input
            type="number"
            id="port"
            value={address.port}
            name="port"
            onChange={handleInput}
          />
        </label>
        <input
          type="submit"
          value="Connect"
          id="button"
          disabled={disable}
          onClick={ connectPort }
        />
      </div>
    </div>
  );
}

export default LoginPort;
