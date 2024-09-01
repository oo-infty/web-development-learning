const form = document.querySelector("form");
const inputUsername = form.querySelector("#username");
const buttonStart = document.querySelector("#button-start");

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
  buttonStart.addEventListener("click", (event) => {
    if (!form.reportValidity()) {
      event.preventDefault();
      return;
    }

    const hour = new Date(Date.now()).getHours();
    let greet = "";

    if (7 < hour && hour < 12) {
      greet = "Good morning";
    } else if (hour < 18) {
      greet = "Good afternoon";
    } else if (hour < 23) {
      greet = "Good evening";
    } else {
      greet = "It may be too late for you to finish the test.";
    }

    const res = window.confirm(`${greet}. The test is about to start. Are you ready?`);

    if (!res) {
      event.preventDefault();
    }
  });
}

function main() {
  registerInputUsername();
  registerButtonStart();
}

main()

