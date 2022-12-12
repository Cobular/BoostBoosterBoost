/* This runs after a web page loads */
{
  const button = document.createElement("button")
  button.addEventListener("click", async () => {
    let res = await fetch("http://localhost:8000/show")
    console.log(await res.text())
  })
  button.textContent = "show"
  document.body.appendChild(button)
}

/* This runs after a web page loads */
{
  const button = document.createElement("button")
  button.addEventListener("click", async () => {
    let res = await fetch("http://localhost:8000/hide")
    console.log(await res.text())
  })
  button.textContent = "hide"
  document.body.appendChild(button)
}

async function messageClient(method, url, body) {
  /** @type RequestInit */
  let options;
  if (body === undefined) {
    options = {
      method: method,
    }
  } else {
    options = {
      method: method,
      body: JSON.stringify(body),
      headers: {
        'Content-Type': 'application/json'
      },
    }
  }
  console.log({ options })
  try {
    let resp = await fetch(`http://localhost:8000/${url}`, options)
    return resp
  } catch (e) {
    console.error(e);
  }

}

/**
 * Checks if the native host is alive
 * 
 * @returns {Promise<boolean>}
 */
async function checkAlive() {
  try {
    const resp = await messageClient("get", "status");
    const resp_text = await resp.text();
    if (resp_text !== "alive") {
      console.error(`Server returned wrong string: ${resp_text}`)
      return false
    }
    return true
  } catch (e) {
    console.error(e)
    return false
  }
}

async function processExtensionClick(link, href) {
  let resp = await messageClient("post", "install", { url: new URL(href, window.location.origin) });
  let text = await resp.text()
  console.log(text)
}

// Link interception logic - https://stackoverflow.com/a/33616981
async function interceptClickEvent(e) {
  async function process_link(evt, link) {
    href = link.getAttribute('href');

    //put your logic here...
    if (href.startsWith("/extensions")) {
      evt.preventDefault();

      const alive = await checkAlive();

      if (alive) {
        console.log("eee")

        // Only prevent the default if we're actually able to connect to the client
        await processExtensionClick(link, href)
      } else {
        alert("Please ensure the Boost Booster Boost native app is running!")
      }

    }
  }

  var target = e.target || e.srcElement;
  const parent_a = target.closest("a")
  if (target.tagName === 'A') {
    process_link(e, target)
  } else if (parent_a !== null) {
    process_link(e, parent_a)
  }
}

//listen for link click events at the document level
if (document.addEventListener) {
  document.addEventListener('click', interceptClickEvent);
} else if (document.attachEvent) {
  document.attachEvent('onclick', interceptClickEvent);
}