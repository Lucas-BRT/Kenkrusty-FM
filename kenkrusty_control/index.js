const paths = {
  sounds: "/sounds",
  playlists: "/playlists",
  mediaboards: "/mediaboards"
};

async function rocketGet(address) {
  try {
    const response = await fetch(address);
    const data = await response.json();
    return data;
  } catch (error) {
    throw new Error(error);
  }
}

function createButton(media) {
  const buttonContainer = document.getElementById('mediaboard');
  const button = document.createElement('button');
  button.textContent = media.title;
  button.className = "mediabutton";
  button.dataset.id = JSON.stringify(media.id);
  button.dataset.media = JSON.stringify(media);

  button.addEventListener('click', () => {
    const id = JSON.parse(button.dataset.id);
    const mediaType = JSON.parse(button.dataset.media).sound_type;
    playMedia(id, mediaType);
  });

  buttonContainer.appendChild(button);
}


function playMedia(id,mediaType) {
  address = `play/${id}/${mediaType}`

  rocketGet(address)

}


class Media {
  constructor(id, title, sound_type, type_commands) {
    this.id = id,
    this.title = title,
    this.sound_type = sound_type,
    this.type_commands = type_commands
  }
}

function sendCommand(command) {
  switch (command) {
    case 'Play':
    break;
  }
}

async function main() {
  let response;

  await rocketGet("/media/Sound")
    .then(data => {
      console.log(data)
      response = data
    }).catch(error => {
      console.log(error)
  });

  const medialist = response.map( media => {
      return new Media(media.id,media.title,media.sound_type,media.type_commands)
    }
  )
  
  medialist.forEach(element => {
    createButton(element)
  });
}

main()





