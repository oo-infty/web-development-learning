const navigationEntry = document.querySelectorAll(".site-navigation a");

const countdown = document.querySelector(".panel-countdown p");
const questionNavigation = document.querySelector(".panel-question-container");

const buttonTestPrevious = document.querySelector(".test-previous button");
const buttonTestNext = document.querySelector(".test-next button");
const buttonTestSubmit = document.querySelector(".test-submit button");
const buttonTestLeave = document.querySelector(".test-leave button");

const questionContainer = document.querySelector(".question-container");

const TOTAL_DURATION_SECONDS = 1800;

let testId;

async function fetchQuestion() {
  try {
    const resp = await fetch("../backend/generate.asp", {
      method: "GET",
      credentials: "same-origin",
    });

    if (resp.ok) {
      const text = await resp.text();
      return text;
    } else {
      return null;
    }
  } catch (error) {
    console.error(`Error: Could not fetch questions from ${url}: ${error}`)
    window.alert("Error: Could not fetch any question. Test aborted");
    location.assign("../index.html");
    return null;
  }
}

function createQuestion(child) {
  const section = document.createElement("section");
  section.setAttribute("class", `question-entry ${child.tagName}`);
  section.setAttribute("id", `question-${child.querySelector("id").textContent}`);

  const h1 = document.createElement("h1");
  h1.setAttribute("class", "question-number");
  section.appendChild(h1);

  const p = document.createElement("p");
  p.textContent = child.querySelector("content").textContent;
  section.appendChild(p);

  if (child.tagName == "single-selection") {
    const answer = createSelectionAnswer(child, "radio");
    section.appendChild(answer);
  } else if (child.tagName == "multiple-selection") {
    const answer = createSelectionAnswer(child, "checkbox");
    section.appendChild(answer);
  } else {
    const answer = createCompletionAnswer(child);
    section.appendChild(answer);
  }

  return section;
}

function createSelectionAnswer(child, componentType) {
  const ul = document.createElement("ul");
  ul.className = "question-answer selection-list";

  Array.from(child.getElementsByTagName("option")).forEach((option, index) => {
    const li = document.createElement("li");

    const id = child.querySelector("id").textContent;
    const input = document.createElement("input");
    input.setAttribute("type", componentType);
    input.setAttribute("id", `option-${id}-${String.fromCharCode(97 + index)}`);
    input.setAttribute("name", `question-${id}`);
    input.setAttribute("value", String.fromCharCode(97 + index));
    li.appendChild(input);

    const label = document.createElement("label");
    label.setAttribute("for", input.getAttribute("id"));
    label.innerHTML = option.textContent;
    li.appendChild(label);

    ul.appendChild(li);
  });

  return ul;
}

function createCompletionAnswer(child) {
  const inputBox = document.createElement("div");
  inputBox.className = "question-answer input-box";

  const id = child.querySelector("id").textContent;
  const input = document.createElement("input");
  input.setAttribute("type", "input");
  input.setAttribute("id", `completion-${id}`);
  input.setAttribute("name", `completion-${id}`);

  inputBox.appendChild(input);
  return inputBox;
}

async function generateQuestion() {
  const questions = await fetchQuestion();

  if (!questions) {
    return;
  }

  const parser = new DOMParser();
  const xml = parser.parseFromString(questions, "application/xml");
  const root = xml.querySelector("root");
  const result = root.querySelector("result").textContent;

  if (result == "not-logined") {
    window.alert("You must login first to participate in the test!");
    location.assign("../login.html");
    return;
  }

  testId = root.querySelector("test-id").textContent;
  const question = root.querySelector("question");

  Array.from(question.children).forEach(child => {
    const question = createQuestion(child);
    questionContainer.appendChild(question);
  });
}

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
  const endInstant = Date.now() + TOTAL_DURATION_SECONDS * 1000;
  let intervalId;

  function updateCounddown() {
    const remainingSeconds = Math.floor((endInstant - Date.now()) / 1000);

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

async function main() {
  await generateQuestion();
  generateQuestionNavigation();

  registerNavigation();
  registerCountdown();
  registerControlButton();
}

main();
