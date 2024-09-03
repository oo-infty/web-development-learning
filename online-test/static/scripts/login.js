const form = document.querySelector("form");
const inputUsername = form.querySelector("#username");
const buttonStart = document.querySelector("#button-start");
const loginMessage = document.querySelector(".login-message");

function registerInputUsername() {
  inputUsername.addEventListener("input", () => {
    inputUsername.setCustomValidity("");
    inputUsername.checkValidity();
  });

  inputUsername.addEventListener("invalid", () => {
    if (inputUsername.value === "") {
      inputUsername.setCustomValidity("A username is required.");
    } else {
      inputUsername.setCustomValidity("A username consists of alphabets, digits, dash and underscore.");
    }
  });
}

function registerButtonStart() {
  async function submit() {
    try {
      const formData = new FormData(form);
      const encodedData = new URLSearchParams(formData).toString();

      const resp = await fetch("TODO", {
        method: "POST",
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body: encodedData,
        credentials: "same-origin",
      });

      return resp.ok;
    } catch (error) {
      console.error(error);
      return false;
    }
  }

  function greetMessage() {
    const hour = new Date(Date.now()).getHours();
    let greet = "";

    if (7 < hour && hour < 12) {
      greet = "Good morning. ";
    } else if (hour < 18) {
      greet = "Good afternoon. ";
    } else if (hour < 23) {
      greet = "Good evening. ";
    } else {
      greet = "";
    }

    return `${greet}You have successfully logined`;
  }

  buttonStart.addEventListener("click", async (event) => {
    event.preventDefault();

    if (!form.reportValidity()) {
      return;
    }

    if (await submit()) {
      const msg = greetMessage();
      loginMessage.textContent = msg;
      loginMessage.setAttribute("class", "login-message login-state-successful");
      setTimeout(() => location.assign("../index.html"), 3000);
    } else {
      loginMessage.textContent = "Failed to login";
      loginMessage.setAttribute("class", "login-message login-state-failed");
    }
  });
}

function main() {
  registerInputUsername();
  registerButtonStart();
}

main()

