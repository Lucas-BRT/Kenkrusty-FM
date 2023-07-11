import React, { useState, useEffect } from 'react';
import './styles.css';
import YellowPlay from '../../assets/YellowPlay.svg';


async function httpGet(address) {
  try {
    const response = await fetch(address);
    const data = await response.json();
    return data;
  } catch (error) {
    throw new Error(error);
  }
};


const PlayButton = (props) => {  
  console.log(props)

  function clickButton() {
    const link = `/play/${props.id}/${props.sound_type}`;
    console.log(link)
    httpGet(link)
  }

  return (
    <button className='MediaButton' onClick={clickButton}>      
      <div className='PlayIconContainer'>
        <img src={YellowPlay} alt="" className='PlayContainer' />
      </div>
      <div className='PlayTextContainer'>
        <span className='PlayText'>
          {props.title}
        </span>        
      </div>
    </button>
  );  
};

const MainPage = () => {

  const [sounds, setSounds] = useState([]);

  useEffect(() => {
    getSoundboard();
  }, []);

  function getSoundboard() {
    httpGet('/media/Sound')
      .then(data => {
        setSounds(data);
      })
      .catch(error => {
        console.log(error);
      });
  };

  return (
    <>
      <div className="SoundsFrame">
        {sounds.map((sound, index) => (
          <PlayButton 
          id={sound.id}
          title={sound.title}
          sound_type={sound.sound_type}
          type_commands={sound.type_commands}
          />
        ))}

      </div>    
    </>
  );
};




export default MainPage;






