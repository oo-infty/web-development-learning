const navigationEntry = document.querySelectorAll(".site-navigation a");

const countdown = document.querySelector(".panel-countdown p");
const questionNavigation = document.querySelector(".panel-question-container");

const buttonTestPrevious = document.querySelector(".test-previous button");
const buttonTestNext = document.querySelector(".test-next button");
const buttonTestSubmit = document.querySelector(".test-submit button");
const buttonTestLeave = document.querySelector(".test-leave button");

const questionContainer = document.querySelector(".question-container");

let remainingSeconds = 1800;

function generateQuestionNavigation() {
  let num = 0;

  for (const question of questionContainer.children) {
    let text = document.createElement("p");
    text.textContent = num;

    let link = document.createElement("a");
    link.setAttribute("id", `button-question-${question.id}`);
    link.appendChild(text);
    const scrollX = questionContainer.clientWidth * num;
    link.addEventListener("click", (event) => {
      questionContainer.scroll(scrollX, 0);
      event.preventDefault();
    });

    let entry = document.createElement("li");
    entry.setAttribute("class", "panel-question-entry");
    entry.appendChild(link);

    questionNavigation.appendChild(entry);

    num += 1;
  }
}

function registerNavigation() {
  for (const entry of navigationEntry) {
    entry.addEventListener("click", (event) => {
      const res = window.confirm("Are you sure to quit the test? All answers will be lost!");

      if (!res) {
        event.preventDefault();
      }
    });
  }
}

function registerCountdown() {
  let intervalId;

  function updateCounddown() {
    if (remainingSeconds === 0) {
      window.alert("The test is over. All answers will be submitted automatically.")
      questionContainer.requestSubmit();

      if (intervalId) {
        clearInterval(intervalId);
      }

      return;
    }

    function padZero(num) {
      return num < 10 ? `0${num}` : num.toString();
    }

    const minutes = Math.floor(remainingSeconds / 60);
    const seconds = remainingSeconds - minutes * 60;
    countdown.textContent = `${padZero(minutes)}:${padZero(seconds)}`;
    remainingSeconds -= 1;
  }

  intervalId = setInterval(updateCounddown, 1000);
  updateCounddown();
}

function registerControlButton() {
  buttonTestPrevious.addEventListener("click", (_event) => {
    questionContainer.scrollBy(-questionContainer.clientWidth, 0)
  });

  buttonTestNext.addEventListener("click", (_event) => {
    questionContainer.scrollBy(questionContainer.clientWidth, 0)
  });

  buttonTestSubmit.addEventListener("click", (_event) => {
    const res = window.confirm("Are you sure to submit the answers?");

    if (res) {
      questionContainer.requestSubmit();
    }
  });

  buttonTestLeave.addEventListener("click", (_event) => {
    const res = window.confirm("Are you sure to quit the test? All answers will be lost!");

    if (res) {
      location.assign("./index.html");
    }
  });
}

function main() {
  generateQuestionNavigation();

  registerNavigation();
  registerCountdown();
  registerControlButton();
}

main();
